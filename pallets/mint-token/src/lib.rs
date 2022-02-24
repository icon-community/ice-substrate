#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;
	use sp_runtime::traits::Saturating;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Storage item for balances to accounts mapping.
	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type BalanceToAccount<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		T::Balance,
		ValueQuery
		>;

	/// Token mint can emit two Event types.
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New token supply was minted.
		MintedNewSupply(T::AccountId),
		/// Tokens were successfully transferred between accounts. [from, to, value]
		Transferred(T::AccountId, T::AccountId, T::Balance),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		pub fn mint(
			origin: OriginFor<T>,
			#[pallet::compact] amount: T::Balance
		) -> DispatchResultWithPostInfo {
			
			let sender = ensure_root(origin)?;
		
			// Update storage.
			<BalanceToAccount<T>>::insert(&sender, amount);

			// Emit an event.
			Self::deposit_event(Event::MintedNewSupply(sender));
			
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}
		
		
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_root(origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances.
			let update_sender = sender_balance.saturating_sub(amount);
			let update_to = receiver_balance.saturating_add(amount);

			// Update both accounts storage.
			<BalanceToAccount<T>>::insert(&sender, update_sender);
			<BalanceToAccount<T>>::insert(&to, update_to);

			// Emit event.
			Self::deposit_event(Event::Transferred(sender, to, amount));
			Ok(().into())
		}
	}
}
