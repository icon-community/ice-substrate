#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
	use frame_support::traits::{OnTimestampSet};

	//Mandatory

	#[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

	//Mandatory
	#[pallet::config]
	pub trait Config: frame_system::Config {

	}

    impl<Moment, T: Config> OnTimestampSet<Moment> for Pallet<T> {
        fn on_timestamp_set(_moment: Moment) {
            log::info!("on_timestamp_set called");
        }
    }
}
