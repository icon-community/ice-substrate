
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-10-175`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("arctic"), DB CACHE: 1024

// Executed Command:
// ./target/release/ice-node
// benchmark
// pallet
// --chain
// arctic
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_vesting
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/arctic/src/weights/pallet_vesting_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 64_668 nanoseconds.
		Weight::from_ref_time(63_696_115)
			// Standard Error: 1_872
			.saturating_add(Weight::from_ref_time(71_457).saturating_mul(l.into()))
			// Standard Error: 3_331
			.saturating_add(Weight::from_ref_time(137_587).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 64_044 nanoseconds.
		Weight::from_ref_time(63_840_650)
			// Standard Error: 1_722
			.saturating_add(Weight::from_ref_time(56_084).saturating_mul(l.into()))
			// Standard Error: 3_065
			.saturating_add(Weight::from_ref_time(89_881).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 66_352 nanoseconds.
		Weight::from_ref_time(64_806_015)
			// Standard Error: 3_123
			.saturating_add(Weight::from_ref_time(75_265).saturating_mul(l.into()))
			// Standard Error: 5_556
			.saturating_add(Weight::from_ref_time(163_682).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 65_601 nanoseconds.
		Weight::from_ref_time(65_210_891)
			// Standard Error: 2_797
			.saturating_add(Weight::from_ref_time(66_456).saturating_mul(l.into()))
			// Standard Error: 4_976
			.saturating_add(Weight::from_ref_time(90_577).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 91_169 nanoseconds.
		Weight::from_ref_time(93_318_403)
			// Standard Error: 4_835
			.saturating_add(Weight::from_ref_time(55_532).saturating_mul(l.into()))
			// Standard Error: 8_602
			.saturating_add(Weight::from_ref_time(76_483).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 92_000 nanoseconds.
		Weight::from_ref_time(94_801_734)
			// Standard Error: 4_696
			.saturating_add(Weight::from_ref_time(53_107).saturating_mul(l.into()))
			// Standard Error: 8_355
			.saturating_add(Weight::from_ref_time(47_652).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 66_297 nanoseconds.
		Weight::from_ref_time(66_750_595)
			// Standard Error: 2_698
			.saturating_add(Weight::from_ref_time(63_632).saturating_mul(l.into()))
			// Standard Error: 4_984
			.saturating_add(Weight::from_ref_time(127_902).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 66_063 nanoseconds.
		Weight::from_ref_time(66_184_797)
			// Standard Error: 2_395
			.saturating_add(Weight::from_ref_time(71_714).saturating_mul(l.into()))
			// Standard Error: 4_424
			.saturating_add(Weight::from_ref_time(124_781).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
