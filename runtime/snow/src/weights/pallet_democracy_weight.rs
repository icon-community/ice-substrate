
//! Autogenerated weights for `pallet_democracy`
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
// pallet_democracy
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/arctic/src/weights/pallet_democracy_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		// Minimum execution time: 87_676 nanoseconds.
		Weight::from_ref_time(89_637_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		// Minimum execution time: 75_290 nanoseconds.
		Weight::from_ref_time(77_043_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 94_871 nanoseconds.
		Weight::from_ref_time(96_476_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		// Minimum execution time: 94_784 nanoseconds.
		Weight::from_ref_time(96_093_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		// Minimum execution time: 38_597 nanoseconds.
		Weight::from_ref_time(39_507_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		// Minimum execution time: 153_243 nanoseconds.
		Weight::from_ref_time(156_659_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		// Minimum execution time: 27_295 nanoseconds.
		Weight::from_ref_time(28_154_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		// Minimum execution time: 8_515 nanoseconds.
		Weight::from_ref_time(8_856_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		// Minimum execution time: 8_557 nanoseconds.
		Weight::from_ref_time(8_906_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		// Minimum execution time: 38_226 nanoseconds.
		Weight::from_ref_time(38_936_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external() -> Weight {
		// Minimum execution time: 45_503 nanoseconds.
		Weight::from_ref_time(46_734_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn cancel_proposal() -> Weight {
		// Minimum execution time: 130_631 nanoseconds.
		Weight::from_ref_time(133_460_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		// Minimum execution time: 25_333 nanoseconds.
		Weight::from_ref_time(25_873_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Minimum execution time: 11_348 nanoseconds.
		Weight::from_ref_time(15_192_381)
			// Standard Error: 6_138
			.saturating_add(Weight::from_ref_time(3_681_919).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Minimum execution time: 15_873 nanoseconds.
		Weight::from_ref_time(19_433_767)
			// Standard Error: 6_326
			.saturating_add(Weight::from_ref_time(3_705_392).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Minimum execution time: 73_641 nanoseconds.
		Weight::from_ref_time(81_481_437)
			// Standard Error: 8_770
			.saturating_add(Weight::from_ref_time(5_565_514).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Minimum execution time: 45_560 nanoseconds.
		Weight::from_ref_time(48_215_662)
			// Standard Error: 8_173
			.saturating_add(Weight::from_ref_time(5_522_846).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		// Minimum execution time: 9_471 nanoseconds.
		Weight::from_ref_time(9_826_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Minimum execution time: 41_638 nanoseconds.
		Weight::from_ref_time(52_535_114)
			// Standard Error: 2_598
			.saturating_add(Weight::from_ref_time(152_357).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Minimum execution time: 49_757 nanoseconds.
		Weight::from_ref_time(52_084_227)
			// Standard Error: 1_489
			.saturating_add(Weight::from_ref_time(202_491).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Minimum execution time: 28_045 nanoseconds.
		Weight::from_ref_time(31_159_277)
			// Standard Error: 1_544
			.saturating_add(Weight::from_ref_time(194_200).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Minimum execution time: 28_408 nanoseconds.
		Weight::from_ref_time(31_443_898)
			// Standard Error: 1_618
			.saturating_add(Weight::from_ref_time(193_913).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
