#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
	use frame_support::traits::{OnTimestampSet};

	//Mandatory
	#[pallet::pallet]
	// require to access the storage of a pallet e.g
	// <Pallet as Store>::Foo.
	#[pallet::generate_store(pub(super) trait Store)]
	// Generates full storage info
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(_);

	//Mandatory
	#[pallet::config]
	pub trait Config: frame_system::Config {

	}

	// Automatically generated if not present
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<Moment, T: Config> OnTimestampSet<Moment> for Pallet<T> {
        fn on_timestamp_set(_moment: Moment) {
            log::info!("on_timestamp_set called");
        }
    }
}
