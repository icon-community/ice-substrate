#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct MetaData<AccountId, Balance> {
	issuance: Balance,
	minter: AccountId,
}

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

use mod inflation_helper;
// Definition of the pallet logic, to be aggregated at runtime definition
// through `construct_runtime`.
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Simple declaration of the `Pallet` type. It is a placeholder we use
	// to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Our pallet's configuration trait. All our types and constants go in here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

		// The minimum balance necessary for an account to exist.
		type MinBalance: Get<Self::Balance>;
	}

	#[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub(super) type MetaDataStore<T: Config> =
		StorageValue<_, MetaData<T::AccountId, T::Balance>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	pub(super) type Accounts<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				admin: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			MetaDataStore::<T>::put(MetaData {
				issuance: Zero::zero(),
				minter: self.admin.clone(),
				burner: self.admin.clone(),
			});
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
	pub enum Event<T: Config> {
		Created(T::AccountId),
		Killed(T::AccountId),
		Minted(T::AccountId, T::Balance),
		Burned(T::AccountId, T::Balance),
		Transfered(T::AccountId, T::AccountId, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		// An account would go below the minimum balance if the operation were executed.
		BelowMinBalance,
		// The origin account does not have the required permission for the operation.
		NoPermission,
		/// An operation would lead to an overflow.
		Overflow,
		/// An operation would lead to an underflow.
		Underflow,
		// Cannot burn the balance of a non-existent account.
		CannotBurnEmpty,
		// There is not enough balance in the sender's account for the transfer.
		InsufficientBalance,
	}

	// You can implement the [`Hooks`] trait to define some logic
	// that should be exectued regularly in some context.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// `on_initialize` is executed at the beginning of the block before any extrinsics are
		// dispatched.
		//
		// This function must return the weight consumed by `on_initialize` and `on_finalize`.
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			// Anything that needs to be done at the start of the block.
			// We don't do anything here.
			let mut meta = MetaDataStore::<T>::get();
			let value: T::Balance = 50u8.into();
			meta.issuance = meta.issuance.saturating_add(value);
			Accounts::<T>::mutate(&meta.minter, |bal| {
				*bal = bal.saturating_add(value);
			});

			0
		}
	}

	// Extrinsics callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn mint(
			origin: OriginFor<T>,
			beneficiary: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(amount >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
			let mut meta = Self::meta_data();
			ensure!(sender == meta.minter, Error::<T>::NoPermission);

			meta.issuance = meta
				.issuance
				.checked_add(&amount)
				.ok_or(Error::<T>::Overflow)?;

			// store the new issuance
			MetaDataStore::<T>::put(meta);

			if Self::increase_balance(&beneficiary, amount) {
				Self::deposit_event(Event::<T>::Created(beneficiary.clone()));
			}
			Self::deposit_event(Event::<T>::Minted(beneficiary, amount));

			Ok(().into())
		}

		#[pallet::weight(1_000)]
		#[transactional]
		pub(super) fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Accounts::<T>::try_mutate(&sender, |bal| -> DispatchResult {
				let new_bal = bal
					.checked_sub(&amount)
					.ok_or(Error::<T>::InsufficientBalance)?;
				ensure!(new_bal >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
				*bal = new_bal;
				Ok(())
			})?;

			Accounts::<T>::try_mutate(&to, |rec_bal| -> DispatchResult {
				let new_bal = rec_bal.saturating_add(amount);
				ensure!(new_bal >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
				*rec_bal = new_bal;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::Transfered(sender, to, amount));

			Ok(().into())
		}
	}
}

// Internal functions of the pallet
impl<T: Config> Pallet<T> {
	fn increase_balance(acc: &T::AccountId, amount: T::Balance) -> bool {
		Accounts::<T>::mutate(&acc, |bal| {
			let created = bal == &Zero::zero();
			// fine because we check the issuance for overflow before minting and transfers
			// don't change the issuance
			*bal = bal.saturating_add(amount);
			created
		})
	}
}

mod inflation_helper {
	//! Helper methods for computing issuance based on inflation
	use crate::pallet::{BalanceOf, Config, Pallet};
	use frame_support::traits::Currency;
	use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
	use scale_info::TypeInfo;
	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};
	use sp_runtime::PerThing;
	use sp_runtime::{Perbill, RuntimeDebug};
	use substrate_fixed::transcendental::pow as floatpow;
	use substrate_fixed::types::{I32F32, I64F64};

	const SECONDS_PER_YEAR: u32 = 31557600;
	const SECONDS_PER_BLOCK: u32 = 6;
	pub const BLOCKS_PER_YEAR: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK;

	fn rounds_per_year<T: Config>() -> u32 {
		let blocks_per_round = <Pallet<T>>::round().length;
		BLOCKS_PER_YEAR / blocks_per_round
	}

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(
		Eq, PartialEq, Clone, Copy, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
	)]
	pub struct Range<T> {
		pub min: T,
		pub ideal: T,
		pub max: T,
	}

	impl<T: Ord> Range<T> {
		pub fn is_valid(&self) -> bool {
			self.max >= self.ideal && self.ideal >= self.min
		}
	}

	impl<T: Ord + Copy> From<T> for Range<T> {
		fn from(other: T) -> Range<T> {
			Range {
				min: other,
				ideal: other,
				max: other,
			}
		}
	}
	/// Convert an annual inflation to a round inflation
	/// round = (1+annual)^(1/rounds_per_year) - 1
	pub fn perbill_annual_to_perbill_round(
		annual: Range<Perbill>,
		rounds_per_year: u32,
	) -> Range<Perbill> {
		let exponent = I32F32::from_num(1) / I32F32::from_num(rounds_per_year);
		let annual_to_round = |annual: Perbill| -> Perbill {
			let x = I32F32::from_num(annual.deconstruct()) / I32F32::from_num(Perbill::ACCURACY);
			let y: I64F64 = floatpow(I32F32::from_num(1) + x, exponent)
				.expect("Cannot overflow since rounds_per_year is u32 so worst case 0; QED");
			Perbill::from_parts(
				((y - I64F64::from_num(1)) * I64F64::from_num(Perbill::ACCURACY))
					.ceil()
					.to_num::<u32>(),
			)
		};
		Range {
			min: annual_to_round(annual.min),
			ideal: annual_to_round(annual.ideal),
			max: annual_to_round(annual.max),
		}
	}
	/// Convert annual inflation rate range to round inflation range
	pub fn annual_to_round<T: Config>(annual: Range<Perbill>) -> Range<Perbill> {
		let periods = rounds_per_year::<T>();
		perbill_annual_to_perbill_round(annual, periods)
	}

	/// Compute round issuance range from round inflation range and current total issuance
	pub fn round_issuance_range<T: Config>(round: Range<Perbill>) -> Range<BalanceOf<T>> {
		let circulating = T::Currency::total_issuance();
		Range {
			min: round.min * circulating,
			ideal: round.ideal * circulating,
			max: round.max * circulating,
		}
	}

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
	pub struct InflationInfo<Balance> {
		/// Staking expectations
		pub expect: Range<Balance>,
		/// Annual inflation range
		pub annual: Range<Perbill>,
		/// Round inflation range
		pub round: Range<Perbill>,
	}

	impl<Balance> InflationInfo<Balance> {
		pub fn new<T: Config>(
			annual: Range<Perbill>,
			expect: Range<Balance>,
		) -> InflationInfo<Balance> {
			InflationInfo {
				expect,
				annual,
				round: annual_to_round::<T>(annual),
			}
		}
		/// Set round inflation range according to input annual inflation range
		pub fn set_round_from_annual<T: Config>(&mut self, new: Range<Perbill>) {
			self.round = annual_to_round::<T>(new);
		}
		/// Reset round inflation rate based on changes to round length
		pub fn reset_round(&mut self, new_length: u32) {
			let periods = BLOCKS_PER_YEAR / new_length;
			self.round = perbill_annual_to_perbill_round(self.annual, periods);
		}
		/// Set staking expectations
		pub fn set_expectations(&mut self, expect: Range<Balance>) {
			self.expect = expect;
		}
	}
}
