use crate::{mock::*, types, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn siganture_validation_valid() {
	let icon_signature = sp_core::bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
	let signed_data = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	let icon_wallet =
		sp_core::bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
	let origin_address = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c"
		.as_bytes()
		.to_vec();

	assert_ok!(AirdropModule::validate_signature(
		&origin_address,
		&icon_wallet,
		&icon_signature,
		&signed_data
	));

	// TODO:
	// Add more sample of valid data in this test
}
