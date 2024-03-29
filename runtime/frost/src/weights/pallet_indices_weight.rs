
//! Autogenerated weights for `pallet_indices`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-10-175`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ice-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_indices
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/frost/src/weights/pallet_indices_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_indices`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_indices::WeightInfo for WeightInfo<T> {
	// Storage: Indices Accounts (r:1 w:1)
	fn claim() -> Weight {
		// Minimum execution time: 45_389 nanoseconds.
		Weight::from_ref_time(46_416_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 55_465 nanoseconds.
		Weight::from_ref_time(56_196_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Indices Accounts (r:1 w:1)
	fn free() -> Weight {
		// Minimum execution time: 47_669 nanoseconds.
		Weight::from_ref_time(48_532_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		// Minimum execution time: 47_604 nanoseconds.
		Weight::from_ref_time(48_311_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Indices Accounts (r:1 w:1)
	fn freeze() -> Weight {
		// Minimum execution time: 54_414 nanoseconds.
		Weight::from_ref_time(55_671_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
