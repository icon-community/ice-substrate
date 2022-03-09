use super::prelude::*;
use core::str::FromStr;
use sp_core::bytes;
use sp_runtime::AccountId32;
use types::{IconVerifiable, SignatureValidationError};

const VALID_ICON_SIGNATURE: &str = "0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900";
const VALID_MESSAGE: &str = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3";
const VALID_ICON_WALLET: types::IconAddress = decode_hex!("ee1448f0867b90e6589289a4b9c06ac4516a75a9");
const VALID_ICE_ADDRESS: &str = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c";

#[test]
fn siganture_validation_valid() {
	{
		let icon_signature = bytes::from_hex(VALID_ICON_SIGNATURE).unwrap();
		let message = VALID_MESSAGE.as_bytes();
		let icon_wallet = VALID_ICON_WALLET;
		let account_id = AccountId32::from_str(VALID_ICE_ADDRESS).unwrap();

		assert_ok!(account_id.verify_with_icon(&icon_wallet, &icon_signature, &message));
	}

	// TODO:
	// add sample of more valid cases
}

#[test]
fn invalid_icon_signature() {
	let icon_wallet = VALID_ICON_WALLET;
	let account_id = AccountId32::from_str(VALID_ICE_ADDRESS).unwrap();

	// When icon address is not in expected format
	{
		let icon_signature = b"this-is-not-expected-length".to_vec();
		assert_err!(
			account_id.verify_with_icon(&icon_wallet, &icon_signature, VALID_MESSAGE.as_bytes()),
			SignatureValidationError::InvalidIconSignature
		);
	}

	// When icon address is in expected format but is invalid
	{
		let icon_signature = bytes::from_hex("0x3a000000002383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f0000000000000000000000000").unwrap();
		assert_err!(
			account_id.verify_with_icon(&icon_wallet, &icon_signature, VALID_MESSAGE.as_bytes()),
			SignatureValidationError::InvalidIconSignature
		);
	}
}

#[test]
fn invalid_ice_address() {
	let icon_signature = bytes::from_hex(VALID_ICON_SIGNATURE).unwrap();
	let icon_wallet =VALID_ICON_WALLET;
	let account_id = AccountId32::from_str(VALID_ICE_ADDRESS).unwrap();

	// Valid message but modified ice_address
	{
		let invalid_account_id = AccountId32::from_str(
			"12345123451234512345e13f522693299b9de1b70ff0464caa5d392396a8f76c",
		)
		.unwrap();
		assert_err!(
			invalid_account_id.verify_with_icon(
				&icon_wallet,
				&icon_signature,
				VALID_MESSAGE.as_bytes()
			),
			SignatureValidationError::InvalidIceAddress
		);
	}

	// Valid ice_address but modified message
	{
		let invalid_message = "icx_sendTransaction.data.{method.transfer.params.{wallet.0000000000000000000000000000000000000000000000000000000000000000}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3";
		assert_err!(
			account_id.verify_with_icon(&icon_wallet, &icon_signature, invalid_message.as_bytes()),
			SignatureValidationError::InvalidIceAddress
		);
	}
}

#[test]
fn invalid_icon_address() {
	let icon_wallet = samples::ICON_ADDRESS[1];
	let account_id = AccountId32::from_str(VALID_ICE_ADDRESS).unwrap();
	let icon_signature = bytes::from_hex(VALID_ICON_SIGNATURE).unwrap();

	assert_err!(
		account_id.verify_with_icon(&icon_wallet, &icon_signature, VALID_MESSAGE.as_bytes()),
		SignatureValidationError::InvalidIconAddress
	);
}

#[test]
fn mock_signature_validation() {
	// It should pass with dummy data, basically anything
	assert_ok!(samples::ACCOUNT_ID[0].verify_with_icon(&[0_u8; 20], &vec![], &vec![]));
}
