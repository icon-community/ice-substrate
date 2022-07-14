use crate as airdrop;
use airdrop::types;
use codec::alloc::string::String;
use frame_support::traits::Get;
use hex::FromHexError;
use sp_core::H160;
use sp_runtime::{
	traits::{BlakeTwo256, Bounded, CheckedDiv, CheckedMul, CheckedSub, Convert, Saturating},
	AccountId32, DispatchError,
};
use sp_std::vec::Vec;

/// Returns an optional vesting schedule which when applied release given amount
/// which will be complete in given block. If
/// Also return amount which is remainder if amount can't be perfectly divided
/// in per block basis
pub fn new_vesting_with_deadline<T, const VESTING_APPLICABLE_FROM: u32>(
	amount: types::VestingBalanceOf<T>,
	ends_in: types::BlockNumberOf<T>,
) -> (Option<types::VestingInfoOf<T>>, types::VestingBalanceOf<T>)
where
	T: pallet_vesting::Config,
{
	const MIN_AMOUNT_PER_BLOCK: u32 = 1u32;
	let min_vesting_amount = <T as pallet_vesting::Config>::MinVestedTransfer::get();

	type BlockToBalance<T> = <T as pallet_vesting::Config>::BlockNumberToBalance;
	let vesting;

	let ends_in_as_balance = BlockToBalance::<T>::convert(ends_in);
	let transfer_over = ends_in_as_balance.saturating_sub(VESTING_APPLICABLE_FROM.into());

	let idol_transfer_multiple = transfer_over * MIN_AMOUNT_PER_BLOCK.into();

	let mut remaining_amount = amount % idol_transfer_multiple;
	let primary_transfer_amount = amount.saturating_sub(remaining_amount);

	let per_block = primary_transfer_amount
		.checked_div(&idol_transfer_multiple)
		.unwrap_or_else(Bounded::min_value);

	let per_block_is_ok = per_block > 0u32.into();
	let locked_amount_is_ok = primary_transfer_amount >= min_vesting_amount;
	if per_block_is_ok && locked_amount_is_ok {
		vesting = Some(types::VestingInfoOf::<T>::new(
			primary_transfer_amount,
			per_block,
			VESTING_APPLICABLE_FROM.into(),
		));
	} else {
		vesting = None;
		remaining_amount = amount;
	}

	(vesting, remaining_amount)
}

pub fn get_instant_percentage<T: airdrop::Config>(is_defi_user: bool) -> u8 {
	if is_defi_user {
		T::VESTING_TERMS.defi_instant_percentage
	} else {
		T::VESTING_TERMS.non_defi_instant_percentage
	}
}

pub fn get_split_amounts<T: airdrop::Config>(
	total_amount: types::BalanceOf<T>,
	instant_percentage: u8,
) -> Result<(types::BalanceOf<T>, types::VestingBalanceOf<T>), DispatchError> {
	let instant_amount = total_amount
		.checked_mul(&instant_percentage.into())
		.ok_or(sp_runtime::ArithmeticError::Overflow)?
		.checked_div(&100_u32.into())
		.ok_or(sp_runtime::ArithmeticError::Underflow)?;

	let vesting_amount = total_amount
		.checked_sub(&instant_amount)
		.ok_or(sp_runtime::ArithmeticError::Underflow)?;

	Ok(
		(
			instant_amount,
			<T::BalanceTypeConversion as Convert<
				types::BalanceOf<T>,
				types::VestingBalanceOf<T>,
			>>::convert(vesting_amount),
		),
	)
}

pub fn recover_address(
	signature: &[u8],
	payload: &[u8],
) -> Result<Vec<u8>, types::SignatureValidationError> {
	use fp_evm::LinearCostPrecompile;
	use pallet_evm_precompile_sha3fips::Sha3FIPS256;
	use pallet_evm_precompile_simple::ECRecoverPublicKey;
	use types::SignatureValidationError;

	const COST: u64 = 1;
	const PADDING_FOR_V: [u8; 31] = [0; 31];

	let (_exit_status, message_hash) =
		Sha3FIPS256::execute(payload, COST).map_err(|_| SignatureValidationError::Sha3Execution)?;
	let formatted_signature = {
		let sig_r = &signature[..32];
		let sig_s = &signature[32..64];
		let sig_v = &signature[64..];

		// Sig final is in the format of:
		// object hash + 31 byte padding + 1 byte v + 32 byte r + 32 byte s
		message_hash
			.iter()
			.chain(&PADDING_FOR_V)
			.chain(sig_v)
			.chain(sig_r)
			.chain(sig_s)
			.cloned()
			.collect::<Vec<u8>>()
	};

	let (_exit_status, recovered_pub_key) = ECRecoverPublicKey::execute(&formatted_signature, COST)
		.map_err(|_| SignatureValidationError::InvalidIconSignature)?;

	let (_exit_status, computed_address) = Sha3FIPS256::execute(&recovered_pub_key, COST)
		.map_err(|_| SignatureValidationError::Sha3Execution)?;
	let address = computed_address[computed_address.len() - 20..].to_vec();

	Ok(address)
}

pub fn into_account_id(address: H160) -> AccountId32 {
	let mut data = [0u8; 24];
	data[0..4].copy_from_slice(b"evm:");
	data[4..24].copy_from_slice(&address[..]);
	let hash = <BlakeTwo256 as sp_runtime::traits::Hash>::hash(&data);
	AccountId32::from(Into::<[u8; 32]>::into(hash))
}

pub fn extract_ice_address(
	payload: &[u8],
	expected_address: &[u8],
) -> Result<Vec<u8>, FromHexError> {
	let expected_string = hex::encode(expected_address);
	let expected_address = expected_string.as_bytes();
	const PREFIX_LEN: usize = b"ice_sendTransaction.data.{method.transfer.params.{wallet.".len();
	let address_len = expected_address.len();
	let slice = payload[PREFIX_LEN..PREFIX_LEN + address_len].to_vec();
	hex::decode(slice)
}

pub fn to_hex_string<T: Clone + Into<Vec<u8>>>(bytes: &T) -> String {
	let vec: Vec<u8> = bytes.clone().into();
	hex::encode(&vec)
}

pub fn hex_as_byte_array<const SIZE: usize>(hex_str: &str) -> Result<[u8; SIZE], FromHexError> {
	let mut bytes = [0u8; SIZE];
	hex::decode_to_slice(hex_str, &mut bytes as &mut [u8])?;
	Ok(bytes)
}

pub fn wrap_bytes(payload: &[u8]) -> Vec<u8> {
	let mut wrapped_message = "<Bytes>".as_bytes().to_vec();
	wrapped_message.extend_from_slice(payload);
	wrapped_message.extend_from_slice("</Bytes>".as_bytes());
	wrapped_message
}

pub fn get_current_block_number<T: frame_system::Config>() -> types::BlockNumberOf<T> {
	<frame_system::Pallet<T>>::block_number()
}
