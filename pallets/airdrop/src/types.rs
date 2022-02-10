use crate as airdrop;
use airdrop::pallet::Config;
use core::convert::Into;
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
use frame_system;
use scale_info::prelude::*;
use scale_info::TypeInfo;
use sp_std::prelude::*;

/// AccountId of anything that implements frame_system::Config
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Type that represent the balance
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

/// Type that represent IconAddress
pub type IconAddress = sp_std::vec::Vec<u8>;

/// type that represnt the error that can occur while validation the signature
#[cfg_attr(feature = "std", derive(Debug, Eq, PartialEq))]
#[cfg_attr(not(feature = "std"), derive(RuntimeDebug))]
pub enum SignatureValidationError {
	InvalidLength,
	InvalidFormat,
	InvalidMessage,
	MismatchedIconAddress,
	MismatchedIceAddress,
	Sha3Execution,
	ECRecoverExecution,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct SnapshotInfo<T: Config> {
	pub icon_address: Vec<u8>,
	pub ice_address: T::AccountId,
	pub amount: BalanceOf<T>,
	pub defi_user: bool,
	pub vesting_percentage: u32,
	pub claim_status: bool,
}

impl<T: Config> SnapshotInfo<T> {
	pub fn icon_address(mut self, val: Vec<u8>) -> Self {
		self.icon_address = val;
		self
	}

	pub fn ice_address(mut self, val: <T as frame_system::Config>::AccountId) -> Self {
		self.ice_address = val;
		self
	}
}

impl<T: Config> Default for SnapshotInfo<T> {
	fn default() -> Self {
		Self {
			ice_address: <T as frame_system::Config>::AccountId::default(),
			icon_address: sp_std::vec![],
			amount: 0_u32.into(),
			defi_user: false,
			vesting_percentage: 0,
			claim_status: false,
		}
	}
}

/// Possible values of error that can occur when doing claim request from offchain worker
pub enum ClaimError {
	/// When there is no icon address in mapping corresponding
	/// to the ice_address stored in queue
	NoIconAddress,

	/// When icon_address do not exists in server database
	NoData,

	/// some error while doing an http request
	HttpError,
}
