
//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_bounties
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/arctic/src/weights/pallet_bounties_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	/// The range of component `d` is `[0, 16384]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Minimum execution time: 51_505 nanoseconds.
		Weight::from_ref_time(53_828_320)
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(1_054).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	fn approve_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn propose_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn unassign_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn accept_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn award_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn claim_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		// Minimum execution time: 60_114 nanoseconds.
		Weight::from_ref_time(61_278_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn close_bounty_active() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn extend_bounty_expiry() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(_b: u32, ) -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(5_737_235)
	}
}
