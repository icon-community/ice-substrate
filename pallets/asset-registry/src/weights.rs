
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_registry.
pub trait WeightInfo {
	fn register_asset() -> Weight;
	fn update_asset_units_per_second() -> Weight;
	fn update_asset_type() -> Weight;
	fn remove_fee_payment_asset() -> Weight;
	fn deregister_asset() -> Weight;
}

/// Weights for pallet_asset_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeId (r:0 w:1)
	fn register_asset() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry AssetTypeId (r:1 w:0)
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	fn update_asset_units_per_second() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:1 w:2)
	// Storage: AssetRegistry AssetTypeId (r:0 w:2)
	fn update_asset_type() -> Weight {
		(69_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	fn remove_fee_payment_asset() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	// Storage: AssetRegistry AssetTypeId (r:0 w:1)
	fn deregister_asset() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeId (r:0 w:1)
	fn register_asset() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry AssetTypeId (r:1 w:0)
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	fn update_asset_units_per_second() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:1 w:2)
	// Storage: AssetRegistry AssetTypeId (r:0 w:2)
	fn update_asset_type() -> Weight {
		(69_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	fn remove_fee_payment_asset() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetRegistry SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetRegistry AssetIdType (r:1 w:1)
	// Storage: AssetRegistry AssetTypeUnitsPerSecond (r:0 w:1)
	// Storage: AssetRegistry AssetTypeId (r:0 w:1)
	fn deregister_asset() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
