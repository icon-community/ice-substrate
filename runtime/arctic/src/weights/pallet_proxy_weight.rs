
//! Autogenerated weights for `pallet_proxy`
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
// pallet_proxy
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/common/src/weights/pallet_proxy_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(75_606_000 as u64)
			// Standard Error: 20_157
			.saturating_add(Weight::from_ref_time(850_239 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(154_523_000 as u64)
			// Standard Error: 12_247
			.saturating_add(Weight::from_ref_time(849_439 as u64).saturating_mul(a as u64))
			// Standard Error: 12_200
			.saturating_add(Weight::from_ref_time(360_609 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(87_138_000 as u64)
			// Standard Error: 16_115
			.saturating_add(Weight::from_ref_time(1_112_953 as u64).saturating_mul(a as u64))
			// Standard Error: 16_053
			.saturating_add(Weight::from_ref_time(719_317 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(109_488_000 as u64)
			// Standard Error: 10_632
			.saturating_add(Weight::from_ref_time(716_189 as u64).saturating_mul(a as u64))
			// Standard Error: 10_592
			.saturating_add(Weight::from_ref_time(220_235 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(131_270_000 as u64)
			// Standard Error: 12_434
			.saturating_add(Weight::from_ref_time(858_956 as u64).saturating_mul(a as u64))
			// Standard Error: 12_386
			.saturating_add(Weight::from_ref_time(488_371 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(109_755_000 as u64)
			// Standard Error: 27_524
			.saturating_add(Weight::from_ref_time(1_194_764 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(116_055_000 as u64)
			// Standard Error: 38_232
			.saturating_add(Weight::from_ref_time(650_889 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		Weight::from_ref_time(98_357_000 as u64)
			// Standard Error: 23_505
			.saturating_add(Weight::from_ref_time(618_748 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		Weight::from_ref_time(128_372_000 as u64)
			// Standard Error: 50_358
			.saturating_add(Weight::from_ref_time(1_346_451 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		Weight::from_ref_time(98_316_000 as u64)
			// Standard Error: 16_997
			.saturating_add(Weight::from_ref_time(747_302 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}