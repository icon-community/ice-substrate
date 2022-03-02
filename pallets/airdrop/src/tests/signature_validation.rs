use super::prelude::*;
use core::str::FromStr;
use sp_core::bytes;
use sp_runtime::AccountId32;
use types::IconVerifiable;

#[test]
fn siganture_validation_valid() {
	{
		let icon_signature = bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
		let message = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
		let icon_wallet = bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
		let origin_address = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c";

		// Verify the this pair is passes the verification
		let account_id = AccountId32::from_str(origin_address).unwrap();
		assert_ok!(account_id.verify_with_icon(&icon_wallet, &icon_signature, &message));
	}

	// TODO:
	// add sample of more valid cases
}

#[test]
fn mock_signature_validation() {
	todo!()
}
