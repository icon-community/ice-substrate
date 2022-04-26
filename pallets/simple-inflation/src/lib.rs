#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	traits::{Currency},
};

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
    pub struct Pallet<T>(PhantomData<T>);

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
		
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	
	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId,>>::NegativeImbalance;

	#[pallet::config]
	pub 
	trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>;

	    #[pallet::constant]
		type IssuingAmount: Get<BalanceOf<Self>>;

		type Beneficiary: Beneficiary<NegativeImbalanceOf<Self>>;
	}
	
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {		
	   fn on_initialize(_now: T::BlockNumber) -> Weight {		 
		 let inflation = T::Currency::issue(T::IssuingAmount::get());
         T::Beneficiary::treasury(inflation);	 		 		 
		 return 0;	
		}
	}
}