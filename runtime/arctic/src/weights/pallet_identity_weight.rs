
//! Autogenerated weights for `pallet_identity`
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
// pallet_identity
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/arctic/src/weights/pallet_identity_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 28_981 nanoseconds.
		Weight::from_ref_time(30_634_415)
			// Standard Error: 4_623
			.saturating_add(Weight::from_ref_time(262_445).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 65_886 nanoseconds.
		Weight::from_ref_time(61_694_207)
			// Standard Error: 4_362
			.saturating_add(Weight::from_ref_time(283_329).saturating_mul(r.into()))
			// Standard Error: 851
			.saturating_add(Weight::from_ref_time(714_742).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 18_059 nanoseconds.
		Weight::from_ref_time(48_893_135)
			// Standard Error: 7_935
			.saturating_add(Weight::from_ref_time(4_585_766).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:2)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 18_178 nanoseconds.
		Weight::from_ref_time(48_264_229)
			// Standard Error: 7_039
			.saturating_add(Weight::from_ref_time(1_982_143).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 98_417 nanoseconds.
		Weight::from_ref_time(62_963_014)
			// Standard Error: 13_639
			.saturating_add(Weight::from_ref_time(273_187).saturating_mul(r.into()))
			// Standard Error: 2_663
			.saturating_add(Weight::from_ref_time(1_895_163).saturating_mul(s.into()))
			// Standard Error: 2_663
			.saturating_add(Weight::from_ref_time(363_178).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 66_362 nanoseconds.
		Weight::from_ref_time(62_720_645)
			// Standard Error: 6_090
			.saturating_add(Weight::from_ref_time(287_913).saturating_mul(r.into()))
			// Standard Error: 1_188
			.saturating_add(Weight::from_ref_time(735_703).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 61_288 nanoseconds.
		Weight::from_ref_time(57_596_225)
			// Standard Error: 4_821
			.saturating_add(Weight::from_ref_time(267_284).saturating_mul(r.into()))
			// Standard Error: 940
			.saturating_add(Weight::from_ref_time(737_507).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 15_243 nanoseconds.
		Weight::from_ref_time(16_088_771)
			// Standard Error: 2_828
			.saturating_add(Weight::from_ref_time(212_454).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 15_482 nanoseconds.
		Weight::from_ref_time(16_431_070)
			// Standard Error: 2_657
			.saturating_add(Weight::from_ref_time(198_565).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 15_136 nanoseconds.
		Weight::from_ref_time(16_234_372)
			// Standard Error: 3_111
			.saturating_add(Weight::from_ref_time(205_827).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 49_240 nanoseconds.
		Weight::from_ref_time(46_134_047)
			// Standard Error: 6_126
			.saturating_add(Weight::from_ref_time(281_870).saturating_mul(r.into()))
			// Standard Error: 1_133
			.saturating_add(Weight::from_ref_time(1_196_797).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 120_989 nanoseconds.
		Weight::from_ref_time(83_572_689)
			// Standard Error: 14_303
			.saturating_add(Weight::from_ref_time(277_929).saturating_mul(r.into()))
			// Standard Error: 2_793
			.saturating_add(Weight::from_ref_time(1_910_649).saturating_mul(s.into()))
			// Standard Error: 2_793
			.saturating_add(Weight::from_ref_time(372_694).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 57_737 nanoseconds.
		Weight::from_ref_time(64_921_592)
			// Standard Error: 2_007
			.saturating_add(Weight::from_ref_time(208_941).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 23_982 nanoseconds.
		Weight::from_ref_time(26_649_140)
			// Standard Error: 1_164
			.saturating_add(Weight::from_ref_time(116_602).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 62_555 nanoseconds.
		Weight::from_ref_time(67_382_960)
			// Standard Error: 1_624
			.saturating_add(Weight::from_ref_time(190_440).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 42_651 nanoseconds.
		Weight::from_ref_time(46_959_147)
			// Standard Error: 1_441
			.saturating_add(Weight::from_ref_time(186_964).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
