
//! Autogenerated weights for `frame_system`
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
// frame_system
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/common/src/weights/frame_system_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		Weight::from_ref_time(14_681_000 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(586 as u64).saturating_mul(b as u64))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		Weight::from_ref_time(27_135_000 as u64)
			// Standard Error: 3
			.saturating_add(Weight::from_ref_time(1_990 as u64).saturating_mul(b as u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(16_094_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(9_465_000 as u64)
			// Standard Error: 2_004
			.saturating_add(Weight::from_ref_time(1_016_080 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(9_487_000 as u64)
			// Standard Error: 2_360
			.saturating_add(Weight::from_ref_time(835_932 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[1, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		Weight::from_ref_time(15_701_000 as u64)
			// Standard Error: 8_875
			.saturating_add(Weight::from_ref_time(1_785_246 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}