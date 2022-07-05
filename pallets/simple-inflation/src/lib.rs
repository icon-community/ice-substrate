#![allow(clippy::unnecessary_cast)]

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	traits::{Currency, Get},
	weights::Weight,
};
use sp_std::marker::PhantomData;

/// We only need to issue inflation to treasury, this will be always set to TreasuryPalletId
pub trait Beneficiary<Imbalance> {
	fn treasury(reward: Imbalance);
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub issuing_amount: BalanceOf<T>,
	}

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::type_value]
	pub fn DefaultIssuingAmount<T: Config>() -> BalanceOf<T> {
		T::IssuingAmount::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn issuing_amount)]
	pub type IssuingAmount<T> = StorageValue<_, BalanceOf<T>, ValueQuery, DefaultIssuingAmount<T>>;

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn new(issuing_amount: BalanceOf<T>) -> Self {
			Self { issuing_amount }
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				issuing_amount: T::IssuingAmount::get(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;

		type IssuingAmount: Get<BalanceOf<Self>>;

		type Beneficiary: Beneficiary<NegativeImbalanceOf<Self>>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_now: T::BlockNumber) -> Weight {
			let inflation = T::Currency::issue(<IssuingAmount<T>>::get());
			T::Beneficiary::treasury(inflation);
			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((
		WeightInfo::<T>::set_config_with_balance(),
		DispatchClass::Operational,
		))]
		pub fn set_issuing_amount(origin: OriginFor<T>, new: BalanceOf<T>) -> DispatchResult {
			ensure_root(origin)?;
			<IssuingAmount<T>>::put(new);
			Ok(())
		}
	}
}

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	pub fn set_config_with_balance() -> Weight {
		(10_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
