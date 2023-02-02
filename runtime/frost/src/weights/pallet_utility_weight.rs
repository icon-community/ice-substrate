
//! Autogenerated weights for `pallet_utility`
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
// pallet_utility
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/frost/src/weights/pallet_utility_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 19_745 nanoseconds.
		Weight::from_ref_time(33_720_133)
			// Standard Error: 3_102
			.saturating_add(Weight::from_ref_time(7_406_573).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 10_770 nanoseconds.
		Weight::from_ref_time(11_231_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 20_036 nanoseconds.
		Weight::from_ref_time(36_680_957)
			// Standard Error: 2_598
			.saturating_add(Weight::from_ref_time(7_699_406).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 23_172 nanoseconds.
		Weight::from_ref_time(23_970_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 19_631 nanoseconds.
		Weight::from_ref_time(28_281_145)
			// Standard Error: 2_444
			.saturating_add(Weight::from_ref_time(7_372_778).saturating_mul(c.into()))
	}
}
