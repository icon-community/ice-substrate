use crate::{Authorship, Balances, FeesSplit, NegativeImbalance, Treasury};
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		if let Some(author) = Authorship::author() {
			Balances::resolve_creating(&author, amount);
		}
	}
}

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		let treasury_cut = FeesSplit::treasury_cut_percent();
		let author_cut = 100 - treasury_cut;

		if let Some(fees) = fees_then_tips.next() {
			let mut split = fees.ration(treasury_cut, author_cut);
			if let Some(tips) = fees_then_tips.next() {
				tips.ration_merge_into(0, 100, &mut split);
			}
			Treasury::on_unbalanced(split.0);
			Author::on_unbalanced(split.1);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::sp_api_hidden_includes_construct_runtime::hidden_include::weights::WeightToFee;
	use crate::{
		constants::{currency::*, time::*},
		AdjustmentVariable, MinimumMultiplier, Runtime, RuntimeBlockWeights, System,
		TargetBlockFullness, TransactionPayment,
	};
	use frame_support::weights::{DispatchClass, Weight};
	use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
	use sp_runtime::{
		assert_eq_error_rate,
		traits::{Convert, One},
		FixedPointNumber,
	};

	use separator::Separatable;

	#[test]
	fn currency_units() {
		println!(
			"DOLLARS 10e18 == {} plank // CENTS == {} plank // MILLICENTS == {} plank",
			DOLLARS.separated_string(),
			CENTS.separated_string(),
			MILLICENTS.separated_string()
		);
		assert!(UNITS < u128::MAX);
		assert_eq!(10u128.pow(18), UNITS);
		assert_eq!(10u128.pow(16), CENTS);
		assert_eq!(10u128.pow(13), MILLICENTS);
	}

	#[test]
	fn block_cost() {
		let max_block_weight = RuntimeBlockWeights::get().max_block;
		let raw_fee: _ =
			<Runtime as pallet_transaction_payment::Config>::WeightToFee::weight_to_fee(
				&max_block_weight,
			);

		println!(
			"Full Block weight == {} // WeightToFee(full_block) == {:?} plank",
			max_block_weight, raw_fee,
		);
	}

	#[test]
	fn full_block_fee_is_correct() {
		let max_block_weight = RuntimeBlockWeights::get().max_block;

		let full_block: u128 =
			<Runtime as pallet_transaction_payment::Config>::WeightToFee::weight_to_fee(
				&max_block_weight,
			);

		assert!(full_block >= 10 * DOLLARS);
		assert!(full_block <= 100 * DOLLARS);
	}

	#[test]
	fn base_fee_is_correct() {
		let extrinsic_base_weight = frame_support::weights::constants::ExtrinsicBaseWeight::get();

		let x: u128 = <Runtime as pallet_transaction_payment::Config>::WeightToFee::weight_to_fee(
			&extrinsic_base_weight,
		);

		let y = CENTS / 10;
		assert!(x.max(y) - x.min(y) < MILLICENTS);
	}

	fn max_normal() -> Weight {
		RuntimeBlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap_or_else(|| RuntimeBlockWeights::get().max_block)
	}

	fn min_multiplier() -> Multiplier {
		MinimumMultiplier::get()
	}

	fn target() -> Weight {
		TargetBlockFullness::get() * max_normal()
	}

	// update based on runtime impl.
	fn runtime_multiplier_update(fm: Multiplier) -> Multiplier {
		TargetedFeeAdjustment::<
			Runtime,
			TargetBlockFullness,
			AdjustmentVariable,
			MinimumMultiplier,
		>::convert(fm)
	}

	// update based on reference impl.
	fn truth_value_update(block_weight: Weight, previous: Multiplier) -> Multiplier {
		let accuracy = Multiplier::accuracy() as f64;
		let previous_float = previous.into_inner() as f64 / accuracy;
		// bump if it is zero.
		let previous_float = previous_float.max(min_multiplier().into_inner() as f64 / accuracy);

		// maximum tx weight
		let m = max_normal() as f64;
		// block weight always truncated to max weight
		let block_weight = (block_weight as f64).min(m);
		let v: f64 = AdjustmentVariable::get().to_float();

		// Ideal saturation in terms of weight
		let ss = target() as f64;
		// Current saturation in terms of weight
		let s = block_weight;

		let t1 = v * (s / m - ss / m);
		let t2 = v.powi(2) * (s / m - ss / m).powi(2) / 2.0;
		let next_float = previous_float * (1.0 + t1 + t2);
		Multiplier::from_float(next_float)
	}

	fn run_with_system_weight<F>(w: Weight, assertions: F)
	where
		F: Fn() -> (),
	{
		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();
		t.execute_with(|| {
			System::set_block_consumed_resources(w, 0);
			assertions()
		});
	}

	#[test]
	fn truth_value_update_poc_works() {
		let fm = Multiplier::saturating_from_rational(1, 2);
		let test_set = vec![
			(0, fm.clone()),
			(100, fm.clone()),
			(1000, fm.clone()),
			(target(), fm.clone()),
			(max_normal() / 2, fm.clone()),
			(max_normal(), fm.clone()),
		];
		test_set.into_iter().for_each(|(w, fm)| {
			run_with_system_weight(w, || {
				assert_eq_error_rate!(
					truth_value_update(w, fm),
					runtime_multiplier_update(fm),
					// Error is only 1 in 100^18
					Multiplier::from_inner(100),
				);
			})
		})
	}

	#[test]
	fn multiplier_can_grow_from_zero() {
		// if the min is too small, then this will not change, and we are doomed forever.
		// the weight is 1/100th bigger than target.
		run_with_system_weight(target() * 101 / 100, || {
			let next = runtime_multiplier_update(min_multiplier());
			assert!(
				next > min_multiplier(),
				"{:?} !>= {:?}",
				next,
				min_multiplier()
			);
		})
	}

	#[test]
	fn multiplier_cannot_go_below_limit() {
		// will not go any further below even if block is empty.
		run_with_system_weight(0, || {
			let next = runtime_multiplier_update(min_multiplier());
			assert_eq!(next, min_multiplier());
		})
	}

	#[test]
	#[ignore] // Takes over 60 sec
	fn time_to_reach_zero() {
		// blocks per 24h in substrate-node: 28,800 (k)
		// s* = 0.1875
		// The bound from the research in an empty chain is:
		// v <~ (p / k(0 - s*))
		// p > v * k * -0.1875
		// to get p == -1 we'd need
		// -1 > 0.00001 * k * -0.1875
		// 1 < 0.00001 * k * 0.1875
		// 10^9 / 1875 < k
		// k > 533_333 ~ 18,5 days.
		run_with_system_weight(0, || {
			// start from 1, the default.
			let mut fm = Multiplier::one();
			let mut iterations: u64 = 0;
			loop {
				let next = runtime_multiplier_update(fm);
				fm = next;
				if fm == min_multiplier() {
					break;
				}
				iterations += 1;
			}
			assert!(iterations > 533_333);
		})
	}

	#[test]
	#[ignore]
	fn congested_chain_simulation() {
		// `cargo test congested_chain_simulation -- --nocapture` to get some insight.

		// almost full. The entire quota of normal transactions is taken.
		let block_weight = RuntimeBlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap() - 100;

		// Default substrate weight.
		let tx_weight = frame_support::weights::constants::ExtrinsicBaseWeight::get();

		run_with_system_weight(block_weight, || {
			// initial value configured on module
			let mut fm = Multiplier::one();
			assert_eq!(fm, TransactionPayment::next_fee_multiplier());

			let mut iterations: u64 = 0;
			loop {
				let next = runtime_multiplier_update(fm);
				// if no change, panic. This should never happen in this case.
				if fm == next {
					panic!("The fee should ever increase");
				}
				fm = next;
				iterations += 1;
				let fee =
					<Runtime as pallet_transaction_payment::Config>::WeightToFee::weight_to_fee(
						&tx_weight,
					);
				let adjusted_fee = fm.saturating_mul_acc_int(fee);
				println!(
					"iteration {}, new fm = {:?}. Fee at this point is: {} units / {} millicents, \
					{} cents, {} dollars",
					iterations,
					fm,
					adjusted_fee,
					adjusted_fee / MILLICENTS,
					adjusted_fee / CENTS,
					adjusted_fee / DOLLARS,
				);
			}
		});
	}

	#[test]
	#[ignore]
	fn min_change_per_day() {
		// This test is failing
		run_with_system_weight(max_normal(), || {
			let mut fm = Multiplier::one();
			// See the example in the doc of `TargetedFeeAdjustment`. are at least 0.234, hence
			// `fm > 1.234`.
			for _ in 0..DAYS {
				let next = runtime_multiplier_update(fm);
				fm = next;
			}
			assert!(fm > Multiplier::saturating_from_rational(1234, 1000));
		})
	}

	#[test]
	fn weight_to_fee_should_not_overflow_on_large_weights() {
		let kb = 1024 as Weight;
		let mb = kb * kb;
		let max_fm = Multiplier::saturating_from_integer(i128::MAX);

		// check that for all values it can compute, correctly.
		vec![
			0,
			1,
			10,
			1000,
			kb,
			10 * kb,
			100 * kb,
			mb,
			10 * mb,
			2147483647,
			4294967295,
			RuntimeBlockWeights::get().max_block / 2,
			RuntimeBlockWeights::get().max_block,
			Weight::MAX / 2,
			Weight::MAX,
		]
		.into_iter()
		.for_each(|i| {
			run_with_system_weight(i, || {
				let next = runtime_multiplier_update(Multiplier::one());
				let truth = truth_value_update(i, Multiplier::one());
				assert_eq_error_rate!(truth, next, Multiplier::from_inner(50_000_000));
			});
		});

		// Some values that are all above the target and will cause an increase.
		let t = target();
		vec![t + 100, t * 2, t * 4].into_iter().for_each(|i| {
			run_with_system_weight(i, || {
				let fm = runtime_multiplier_update(max_fm);
				// won't grow. The convert saturates everything.
				assert_eq!(fm, max_fm);
			})
		});
	}
}
