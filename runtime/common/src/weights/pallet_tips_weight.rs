
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `sabin-Inspiron-7559`, CPU: `Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz`
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
// pallet_tips
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/common/src/weights/pallet_tips_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(126_779_000 as u64)
			// Standard Error: 111
			.saturating_add(Weight::from_ref_time(5_946 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(130_870_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 7]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(93_183_000 as u64)
			// Standard Error: 31
			.saturating_add(Weight::from_ref_time(4_602 as u64).saturating_mul(r as u64))
			// Standard Error: 75_287
			.saturating_add(Weight::from_ref_time(1_487_396 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(60_966_000 as u64)
			// Standard Error: 29_886
			.saturating_add(Weight::from_ref_time(1_805_849 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn close_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(192_998_000 as u64)
			// Standard Error: 39_306
			.saturating_add(Weight::from_ref_time(1_572_864 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 7]`.
	fn slash_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(75_372_000 as u64)
			// Standard Error: 22_263
			.saturating_add(Weight::from_ref_time(819_014 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
