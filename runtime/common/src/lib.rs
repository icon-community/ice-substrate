#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

const MAX_PERCENT: u32 = 100;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct HostConfiguration {
	pub treasury_fee_cut_percent: u32,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type HostConfig: Get<HostConfiguration>;
	}

	#[pallet::type_value]
	pub fn DefaultHostConfig<T: Config>() -> HostConfiguration {
		T::HostConfig::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn config)]
	pub type ActiveConfig<T> = StorageValue<_, HostConfiguration, ValueQuery, DefaultHostConfig<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((
        WeightInfo::<T>::set_config_with_u32(),
        DispatchClass::Operational,
        ))]
		pub fn set_treasury_fee_cut_percent(origin: OriginFor<T>, new: u32) -> DispatchResult {
			ensure_root(origin)?;
			let mut config = <ActiveConfig<T>>::get();

			if new <= MAX_PERCENT {
				config.treasury_fee_cut_percent = new;
			}

			<ActiveConfig<T>>::set(config);
			Ok(())
		}
	}
}

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: Config> WeightInfo<T> {
	pub fn set_config_with_u32() -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
