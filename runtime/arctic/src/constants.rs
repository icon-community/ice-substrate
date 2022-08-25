#![allow(clippy::identity_op)]

pub mod currency {
	use crate::Balance;

	/// The existential deposit.
	pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

	pub const UNITS: Balance = 1_000_000_000_000_000_000; // 1.0 ICY = 10e18 Planck
	pub const DOLLARS: Balance = UNITS;
	pub const CENTS: Balance = DOLLARS / 100; // 0.01 ICY = 10e16 Planck
	pub const MILLICENTS: Balance = CENTS / 1000; // 0.001 ICY = 10e15 Planck
	pub const MICROCENTS: Balance = MILLICENTS / 1000; // 0.0001 ICY = 10e13 Planck

	/// Constant values for the base number of indivisible units for balances
	pub const MILLIICY: Balance = MILLICENTS;
	pub const ICY: Balance = UNITS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 1 * DOLLARS + (bytes as Balance) * 5 * MILLICENTS
	}
}

/// Time and blocks.
pub mod time {
	type Moment = u64;
	use crate::BlockNumber;

	pub const MILLISECS_PER_BLOCK: Moment = 12000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	//pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = SLOT_DURATION;//prod_or_fast!(1 * HOURS, 1 * MINUTES);

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

/// Fee-related.
pub mod fee {
	use crate::{Balance, ExtrinsicBaseWeight};
	use frame_support::weights::constants::WEIGHT_PER_SECOND;
	use frame_support::weights::{
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, `MAXIMUM_BLOCK_WEIGHT`]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Arctic, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = base_tx_in_icz();
			let q = Balance::from(ExtrinsicBaseWeight::get());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}

	pub fn base_tx_in_icz() -> Balance {
		super::currency::CENTS / 10
	}

	pub fn icz_per_second() -> u128 {
		let base_weight = Balance::from(ExtrinsicBaseWeight::get());
		let base_tx_per_second = (WEIGHT_PER_SECOND as u128) / base_weight;
		base_tx_per_second * base_tx_in_icz()
	}

	pub fn ksm_per_second() -> u128 {
		icz_per_second() / 1_000_000 / 2_500
	}
}
