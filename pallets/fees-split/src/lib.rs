#![allow(clippy::unnecessary_cast)]

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

const MAX_PERCENT: u32 = 100;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub treasury_cut_percent: u32,
		_marker: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn new(treasury_cut_percent: u32) -> Self {
			Self {
				treasury_cut_percent,
				_marker: PhantomData,
			}
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				treasury_cut_percent: 80,
				_marker: PhantomData,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<TreasuryCutPercent<T>>::put(self.treasury_cut_percent);
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn treasury_cut_percent)]
	pub type TreasuryCutPercent<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((
		WeightInfo::<T>::set_config_with_u32(),
		DispatchClass::Operational,
		))]
		pub fn set_treasury_cut_percent(origin: OriginFor<T>, val: u32) -> DispatchResult {
			ensure_root(origin)?;

			if val <= MAX_PERCENT {
				<TreasuryCutPercent<T>>::set(val);
			}

			Ok(())
		}
	}
}

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: Config> WeightInfo<T> {
	pub fn set_config_with_u32() -> Weight {
		(10_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
