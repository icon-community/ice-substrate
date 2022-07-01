use crate as airdrop;
use airdrop::pallet::Config;
use airdrop::pallet::Error;
use codec::MaxEncodedLen;
use core::convert::Into;
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
use frame_system;
use scale_info::TypeInfo;
use serde::Deserialize;
use sp_core::H160;
use sp_runtime::traits::Convert;
use sp_runtime::ArithmeticError;
use sp_std::prelude::*;

use frame_support::storage::bounded_vec::BoundedVec;

/// AccountId of anything that implements frame_system::Config
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

///
pub type VestingBalanceOf<T> =
	<<T as pallet_vesting::Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

/// Type that represent the balance
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

pub type ServerBalance = u128;

pub fn to_balance<T: Config>(amount: ServerBalance) -> BalanceOf<T> {
	<T::BalanceTypeConversion as Convert<ServerBalance, BalanceOf<T>>>::convert(amount)
}

pub fn from_balance<T: Config>(amount: BalanceOf<T>) -> ServerBalance {
	<T::BalanceTypeConversion as Convert<BalanceOf<T>, ServerBalance>>::convert(amount)
}

/// Type that represent IconAddress
pub type IconAddress = [u8; 20];

pub type IceAddress = [u8; 32];

pub type IceEvmAddress = H160;

/// Type that represent Icon signed message
pub type IconSignature = [u8; 65];

//
pub type IceSignature = [u8; 64];

//
pub type RawPayload = [u8; RAW_PAYLOAD_LENGTH];

///
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

pub type MerkleHash = [u8; 32];
// pub type MerkleProofs=Vec<MerkleHash>;
pub type MerkleProofs<T> = BoundedVec<MerkleHash, <T as Config>::MaxProofSize>;

///
pub type VestingInfoOf<T> = pallet_vesting::VestingInfo<VestingBalanceOf<T>, BlockNumberOf<T>>;

/// type that represnt the error that can occur while validation the signature
#[derive(Eq, PartialEq, Debug)]
pub enum SignatureValidationError {
	InvalidIconAddress,
	InvalidIconSignature,
	InvalidIceAddress,
	Sha3Execution,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct SnapshotInfo<T: Config> {
	/// Icon address of this snapshot
	pub ice_address: AccountIdOf<T>,

	/// Total airdroppable-amount this icon_address hold
	pub amount: BalanceOf<T>,

	/// Indicator wather this icon_address holder is defi-user
	pub defi_user: bool,

	/// indicator wather the user have claimmed the balance
	/// which will be given through instant transfer
	pub done_instant: bool,

	/// Indicator weather vesting schedult have been applied
	/// to this user
	pub done_vesting: bool,

	// blocknumber that started vesting
	pub vesting_block_number: Option<BlockNumberOf<T>>,

	// block number when instant amount was given
	pub instant_block_number: Option<BlockNumberOf<T>>,

	pub initial_transfer: BalanceOf<T>,
}

impl<T: Config> core::fmt::Debug for SnapshotInfo<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SnapshotInfo")
			.field("ice_address", &self.ice_address)
			.field("amount", &self.amount)
			.field("defi_user", &self.defi_user)
			.field("done_instant", &self.done_instant)
			.field("done_vesting", &self.done_vesting)
			.field("vesting_block_number", &self.vesting_block_number)
			.field("initial_transfer", &self.initial_transfer)
			.field("instant_block_number", &self.instant_block_number)
			.finish()
	}
}

impl<T: Config> SnapshotInfo<T> {
	pub fn new(ice_address: AccountIdOf<T>, defi_user: bool, amount: BalanceOf<T>) -> Self {
		SnapshotInfo::<T> {
			ice_address,
			amount,
			defi_user,
			done_instant: false,
			done_vesting: false,
			vesting_block_number: None,
			instant_block_number: None,
			initial_transfer: 0u32.into(),
		}
	}
}

impl<T: Config> From<ArithmeticError> for Error<T> {
	fn from(_: ArithmeticError) -> Self {
		Error::<T>::ArithmeticError
	}
}

impl<T: Config> From<SignatureValidationError> for Error<T> {
	fn from(_: SignatureValidationError) -> Self {
		Error::<T>::InvalidSignature
	}
}

pub fn balance_to_u32<T: Config>(input: BalanceOf<T>) -> u32 {
	TryInto::<u32>::try_into(input).ok().unwrap()
}

pub fn block_number_to_u32<T: Config>(input: BlockNumberOf<T>) -> u32 {
	TryInto::<u32>::try_into(input).ok().unwrap()
}
/// Chain state
#[derive(Deserialize, Encode, Decode, Clone, Eq, PartialEq, TypeInfo, MaxEncodedLen, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct AirdropState {
	// Only receive claim request when this flag is true
	pub block_claim_request: bool,

	// Only receive exchange request when this flag is true
	pub block_exchange_request: bool,
}

impl Default for AirdropState {
	#[cfg(not(test))]
	fn default() -> Self {
		AirdropState {
			block_claim_request: true,
			block_exchange_request: true,
		}
	}

	#[cfg(test)]
	fn default() -> Self {
		AirdropState {
			block_claim_request: false,
			block_exchange_request: false,
		}
	}
}

pub trait MerkelProofValidator<T: Config> {
	fn validate(leaf_hash: MerkleHash, root_hash: MerkleHash, proofs: MerkleProofs<T>) -> bool;
}

/// Trait to commit behaviour of do_transfer function
/// this trait now can me implmeneted according to
/// the node behaviour eg: vesting manner and direct manner
pub trait DoTransfer {
	fn do_transfer<T: Config>(snapshot: &mut SnapshotInfo<T>) -> DispatchResult;
}

pub struct AirdropBehaviour {
	pub defi_instant_percentage: u8,
	pub non_defi_instant_percentage: u8,
	pub vesting_period: u32,
}

pub const RAW_PAYLOAD_LENGTH: usize = b"icx_sendTransaction.data.{method.transfer.params.{wallet.b6e7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48}}.dataType.call.from.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.version.0x3".len();
