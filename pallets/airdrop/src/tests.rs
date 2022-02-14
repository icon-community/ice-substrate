use crate as pallet_airdrop;
use crate::{
	mock::{self, AirdropModule, Test},
	types, Error,
};
use frame_support::{assert_err, assert_noop, assert_ok};

/// Sample icon address when sent to server retrurning defined response
const PREDEFINED_REQUEST_RESPONSE: (&str, &str) = (
	"0x000000000000000000000000000000",
	r#"{"omm":0,"balanced":300,"stake":464764,"defi_user":true}"#,
);

/// A helper macro that will return the required variable to start testing offchain logic
// TODO:
// Convert this macro to function
macro_rules! new_offchain_test_ext {
	() => {{
		use sp_core::offchain::TransactionPoolExt;
		use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
		use sp_runtime::RuntimeAppPublic;
		use std::sync::Arc;

		const PHRASE: &str =
			"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
		let keystore = KeyStore::new();
		SyncCryptoStore::sr25519_generate_new(
			&keystore,
			crate::temporary::Public::ID,
			Some(&format!("{}/abcdefg", PHRASE)),
		)
		.unwrap();

		let mut test_ext = sp_io::TestExternalities::default();
		let (pool, pool_state) = sp_core::offchain::testing::TestTransactionPoolExt::new();
		let (offchain, state) = sp_core::offchain::testing::TestOffchainExt::new();

		test_ext.register_extension(sp_core::offchain::OffchainWorkerExt::new(offchain));
		test_ext.register_extension(TransactionPoolExt::new(pool));
		test_ext.register_extension(KeystoreExt(Arc::new(keystore)));

		(test_ext, state)
	}};
}

#[test]
fn siganture_validation_valid() {
	let icon_signature = sp_core::bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
	let signed_data = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	let icon_wallet =
		sp_core::bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
	let origin_address = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c"
		.as_bytes()
		.to_vec();

	assert_ok!(mock::AirdropModule::validate_signature(
		&origin_address,
		&icon_wallet,
		&icon_signature,
		&signed_data
	));

	// TODO:
	// Add more sample of valid data in this test
}

#[test]
fn siganture_validation_invalid() {
	let icon_signature = sp_core::bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
	let signed_data = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	let icon_wallet =
		sp_core::bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
	let origin_address = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c"
		.as_bytes()
		.to_vec();

	let invalid_icon_signature = sp_core::bytes::from_hex("0x11111111112383d60e1d9d95763cf4b5555555555daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50000000000").unwrap();
	let invalid_signed_data = "icx_sendTransaction.data.{method.transfer.params.{wallet.9999999999c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	let invalid_icon_wallet =
		sp_core::bytes::from_hex("0xee1448f0877660e6589289a4b00000c451777777").unwrap();
	let invalid_origin_address = "0000b20713c087e12abae13f522693299b9de1b70ff0464caa5d390000000000"
		.as_bytes()
		.to_vec();

	let should_be_invalid_ice_address = mock::AirdropModule::validate_signature(
		&origin_address,
		&icon_wallet,
		&icon_signature,
		&invalid_signed_data,
	);

	let should_be_invalid_icon_address = mock::AirdropModule::validate_signature(
		&origin_address,
		&invalid_icon_wallet,
		&icon_signature,
		&signed_data,
	);

	let should_be_invalid_icon_signature = mock::AirdropModule::validate_signature(
		&origin_address,
		&icon_wallet,
		&icon_signature[10..],
		&signed_data,
	);

	assert_eq!(
		should_be_invalid_ice_address.unwrap_err(),
		types::SignatureValidationError::InvalidIceAddress
	);
	assert_eq!(
		should_be_invalid_icon_address.unwrap_err(),
		types::SignatureValidationError::InvalidIconAddress
	);
	assert_eq!(
		should_be_invalid_icon_signature.unwrap_err(),
		types::SignatureValidationError::InvalidIconSignature
	);
}

#[test]
fn making_http_request() {
	let (mut test_ext, state) = new_offchain_test_ext! {};
	put_response(&mut state.write());

	test_ext.execute_with(|| {
		let icon_address = sp_core::bytes::from_hex(PREDEFINED_REQUEST_RESPONSE.0).unwrap();
		let fetch_res = mock::AirdropModule::fetch_from_server(icon_address);
		assert_ok!(fetch_res);
	});
}

#[test]
fn process_claim_invalid() {
	let (mut test_ext, state) = new_offchain_test_ext! {};

	test_ext.execute_with(|| {
		let ice_address = types::AccountIdOf::<Test>::default();

		// Nothing is in queue yet so, should fail with no_icon_address
		let no_icon_address = AirdropModule::process_claim_request(ice_address.clone());
		assert_eq!(
			no_icon_address.unwrap_err(),
			types::ClaimError::NoIconAddress
		);
	});
}

#[test]
fn process_claim_valid() {
	let (mut test_ext, state) = new_offchain_test_ext!();

	put_response(&mut state.write());

	test_ext.execute_with(|| {
		let ice_address = types::AccountIdOf::<Test>::default();
		let icon_address = sp_core::bytes::from_hex(PREDEFINED_REQUEST_RESPONSE.0).unwrap();
		let snapshot = types::SnapshotInfo::<Test>::default();

		crate::IceSnapshotMap::insert(ice_address, snapshot.clone().icon_address(icon_address));

		let should_be_ok = AirdropModule::process_claim_request(ice_address.clone());
		assert_ok!(should_be_ok);
	});
}

#[test]
fn test_transfer_valid() {
	mock::new_test_ext().execute_with(|| {
		let system_account_id = AirdropModule::get_creditor_account();
		let claimer = types::AccountIdOf::<Test>::default();

		// Set some balance to creditor account first
		let diposit_res = <Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::Origin::root(),
			system_account_id.clone(),
			10_00_000_u32.into(),
			10_000_u32.into(),
		);
		assert_ok!(diposit_res);

		// Simulate that we have done a claim_request by adding it to PendingClaims queue
		pallet_airdrop::PendingClaims::<Test>::insert(&claimer, ());

		// Check for sucess transfer. Ensure that receiver balances is credited and system balances is debited
		{
			use frame_support::traits::tokens::currency::Currency;

			let root_origin = mock::Origin::root();
			let server_response = types::ServerResponse {
				omm: 123_u32.into(),
				amount: 10_000_u32.into(),
				stake: 12_u32.into(),
				defi_user: true,
			};

			let pre_system_balance =
				<Test as pallet_airdrop::Config>::Currency::free_balance(&system_account_id);
			let pre_user_balance =
				<Test as pallet_airdrop::Config>::Currency::free_balance(&claimer);

			let transfer_res = AirdropModule::transfer_amount(
				mock::Origin::root(),
				claimer.clone(),
				server_response,
			);
			assert_ok!(transfer_res);

			let post_system_balance =
				<Test as pallet_airdrop::Config>::Currency::free_balance(&system_account_id);
			let post_user_balance =
				<Test as pallet_airdrop::Config>::Currency::free_balance(&claimer);

			// Make sure user got right amount of money
			assert_eq!(post_user_balance, server_response.amount.into());

			// System balance is only requeced by balance transferred only as fee is 0 for this call
			assert_eq!(
				post_system_balance,
				pre_system_balance - server_response.amount
			);

			// Make sure that net sum of node remains same.
			// i.e fund is not lost anywhere
			assert_eq!(
				pre_system_balance + pre_user_balance,
				post_system_balance + post_user_balance
			);

			// Make sure that request is removed from queue after transfer
			assert_eq!(
				pallet_airdrop::PendingClaims::<Test>::contains_key(&claimer),
				false
			);
		}
	});
}

#[test]
fn test_transfer_invalid() {
	mock::new_test_ext().execute_with(|| {
		let server_response = types::ServerResponse::default();
		let receiver = types::AccountIdOf::<Test>::default();

		// Try to claim something when the data is not in queue
		// simulate the condition when user had cancelled the claim while process was goingon in offchain
		{
			let root_origin = mock::Origin::root();
			let fail_with_absent_queue = AirdropModule::transfer_amount(
				root_origin.clone(),
				receiver.clone(),
				server_response.clone(),
			);
			assert_eq!(
				fail_with_absent_queue.unwrap_err(),
				crate::Error::<Test>::IncompleteData.into()
			);
		}

		// Try to call this function with unauthorised key
		{
			let unauthorised_user = mock::Origin::signed(types::AccountIdOf::<Test>::default());
			let fail_with_permission = AirdropModule::transfer_amount(
				unauthorised_user,
				receiver.clone(),
				server_response.clone(),
			);
			assert_eq!(
				fail_with_permission.unwrap_err(),
				crate::Error::<Test>::DeniedOperation.into()
			);
		}
	});
}

use sp_core::offchain::testing;
/// Helper function to initialise PendingResult struct as per passed by (icon_address & response)
fn put_response(state: &mut testing::OffchainState) {
	let uri = mock::FetchIconEndpoint::get()
		.as_bytes()
		.iter()
		.chain(PREDEFINED_REQUEST_RESPONSE.0.as_bytes())
		.cloned()
		.collect::<Vec<u8>>();
	let uri = String::from_utf8(uri).unwrap();
	let method = "GET".to_string();
	let response = Some(PREDEFINED_REQUEST_RESPONSE.1.as_bytes().to_vec());

	state.expect_request(testing::PendingRequest {
		method,
		uri,
		response,
		sent: true,
		..Default::default()
	});
}
