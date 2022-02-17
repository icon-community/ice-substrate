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

/// Test that the signature validation passed against the same AccountId type as used in real runtime
/// This method will always passed when tested against with mock runtime
/// So, it is tested against the concrete AccountId32 type ( which is the AccountId type in real runtime)
#[test]

fn siganture_validation_valid() {
	use core::str::FromStr;
	use types::IconVerifiable;

	// New sample
	{
		let icon_signature = sp_core::bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
		let message = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
		let icon_wallet =
			sp_core::bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
		let origin_address = "da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c";

		// Verify the this pair is passes the verification
		let account_id = sp_runtime::AccountId32::from_str(origin_address).unwrap();
		assert_ok!(account_id.verify_with_icon(&icon_wallet, &icon_signature, &message));
	}

	// TODO:
	// Add more sample of valid data in this test
}

/// See: docs from siganture_validation_valid
#[test]
fn siganture_validation_invalid() {
	use core::str::FromStr;
	use types::IconVerifiable;

	let icon_signature = sp_core::bytes::from_hex("0x628af708622383d60e1d9d95763cf4be64d0bafa8daebb87847f14fde0db40013105586f0c937ddf0e8913251bf01cf8e0ed82e4f631b666453e15e50d69f3b900").unwrap();
	let message = "icx_sendTransaction.data.{method.transfer.params.{wallet.da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	// Message with unmatching ice_address
	let invalid_message = "icx_sendTransaction.data.{method.transfer.params.{wallet.0000000000000000000000000000000000000000000000000000000000000000}}.dataType.call.from.hxdd9ecb7d3e441d25e8c4f03cd20a80c502f0c374.nid.0x1.nonce.0x1..timestamp.0x5d56f3231f818.to.cx8f87a4ce573a2e1377545feabac48a960e8092bb.version.0x3".to_string().as_bytes().to_vec();
	let icon_wallet =
		sp_core::bytes::from_hex("0xee1448f0867b90e6589289a4b9c06ac4516a75a9").unwrap();
	let ice_address = sp_runtime::AccountId32::from_str(
		"da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c",
	)
	.unwrap();
	let invalid_ice_address = types::AccountIdOf::<Test>::default();

	// Passing invalid message will fail with invalid_ice_addr
	let should_be_invalid_ice_address =
		ice_address.verify_with_icon(&icon_wallet, &icon_signature, &invalid_message);

	// Passing invalid icon signature
	let should_be_invalid_icon_signature =
		ice_address.verify_with_icon(&icon_wallet, b"invalid-icon-signature", &message);

	// Passing invalid icon_address
	let should_be_invalid_icon_address =
		ice_address.verify_with_icon(&b"invalid-icon-address".to_vec(), &icon_signature, &message);

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
		// Add Dummy snapshot to mapping
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&claimer,
			types::SnapshotInfo::<Test>::default(),
		);

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

			let transfer_res =
				AirdropModule::complete_transfer(root_origin, claimer.clone(), server_response);
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
			assert_eq!(AirdropModule::get_pending_claims(&claimer), None);
			// Make sure this function update the snapshot mapping
			assert_eq!(
				AirdropModule::get_ice_snapshot_map(&claimer)
					.unwrap()
					.claim_status,
				true
			);
		}
	});
}

#[test]
fn test_ensure_root_or_sudo() {
	mock::new_test_ext().execute_with(|| {
		use frame_system::ensure_signed;
		use sp_runtime::DispatchError::BadOrigin;

		let sudo_origin = mock::Origin::signed(AirdropModule::get_sudo_account());
		let signed_origin = mock::Origin::signed(sp_core::sr25519::Public([12; 32]));
		let root_origin = mock::Origin::root();
		let unsigned_origin = mock::Origin::none();

		assert_ne!(
			ensure_signed(sudo_origin.clone()).unwrap(),
			ensure_signed(signed_origin.clone()).unwrap(),
			"Non-sudo and sudo origin are same. Change test value"
		);

		let sudo_call = AirdropModule::ensure_root_or_sudo(sudo_origin);
		let root_call = AirdropModule::ensure_root_or_sudo(root_origin);
		let signed_call = AirdropModule::ensure_root_or_sudo(signed_origin);
		let unsigned_call = AirdropModule::ensure_root_or_sudo(unsigned_origin);

		assert_ok!(sudo_call);
		assert_ok!(root_call);
		assert_err!(signed_call, BadOrigin);
		assert_err!(unsigned_call, BadOrigin);
	});
}

#[test]
fn test_cancel_claim() {
	mock::new_test_ext().execute_with(|| {
		// Unsigned origin is not allowed
		{
			let res_denied = AirdropModule::cancel_claim_request(
				mock::Origin::none(),
				types::AccountIdOf::<Test>::default(),
			);

			assert_err!(res_denied, Error::<Test>::DeniedOperation);
		}

		// Signed but not sudo nor owner
		{
			let signed_origin = mock::Origin::signed(sp_core::sr25519::Public([12; 32]));
			let res_denied = AirdropModule::cancel_claim_request(
				signed_origin,
				types::AccountIdOf::<Test>::default(),
			);

			assert_err!(res_denied, Error::<Test>::DeniedOperation);
		}

		// When entry to remove is not in queue
		{
			let caller_id = sp_core::sr25519::Public([12; 32]);
			let signed_origin = mock::Origin::signed(caller_id.clone());
			let res_no_data = AirdropModule::cancel_claim_request(signed_origin, caller_id);

			assert_err!(res_no_data, Error::<Test>::NotInQueue);
		}

		// Should pass when owner of claimer calls
		{
			let caller_id = sp_core::sr25519::Public([12; 32]);
			let signed_origin = mock::Origin::signed(caller_id.clone());
			pallet_airdrop::PendingClaims::<Test>::insert(&caller_id, ());
			let ok_with_owner =
				AirdropModule::cancel_claim_request(signed_origin.clone(), caller_id.clone());

			assert_ok!(ok_with_owner);
			assert_eq!(AirdropModule::get_pending_claims(&caller_id), None);
		}

		// Should pass when root calls in
		{
			let caller_id = sp_core::sr25519::Public([12; 32]);
			pallet_airdrop::PendingClaims::<Test>::insert(&caller_id, ());
			let ok_with_root =
				AirdropModule::cancel_claim_request(mock::Origin::root(), caller_id.clone());

			assert_ok!(ok_with_root);
			assert_eq!(AirdropModule::get_pending_claims(&caller_id), None);
		}

		// Should pass when sudo calls in
		{
			let caller_id = sp_core::sr25519::Public([12; 32]);
			let sudo_origin = mock::Origin::signed(caller_id.clone());
			pallet_airdrop::PendingClaims::<Test>::insert(&caller_id, ());
			let ok_with_root = AirdropModule::cancel_claim_request(sudo_origin, caller_id.clone());

			assert_ok!(ok_with_root);
			assert_eq!(AirdropModule::get_pending_claims(&caller_id), None);
		}
	});
}

#[test]
fn test_transfer_invalid() {
	mock::new_test_ext().execute_with(|| {
		let server_response = types::ServerResponse::default();
		let receiver = sp_core::sr25519::Public([200; 32]);

		// Try to claim something when the data is not in queue
		// simulate the condition when user had cancelled the claim while process was goingon in offchain
		{
			let root_origin = mock::Origin::root();
			let fail_with_absent_queue = AirdropModule::complete_transfer(
				root_origin.clone(),
				receiver.clone(),
				server_response.clone(),
			);
			assert_eq!(
				fail_with_absent_queue.unwrap_err(),
				crate::Error::<Test>::NotInQueue.into()
			);
		}

		// Try to claim when data is in queue but not in map
		{
			// Add this to queue
			crate::PendingClaims::<Test>::insert(&receiver, ());

			let root_origin = mock::Origin::root();
			let fail_with_absent_queue = AirdropModule::complete_transfer(
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
			let unauthorised_user = mock::Origin::signed(sp_core::sr25519::Public([10; 32]));
			let fail_with_permission = AirdropModule::complete_transfer(
				unauthorised_user,
				receiver.clone(),
				server_response.clone(),
			);
			assert_eq!(
				fail_with_permission.unwrap_err(),
				crate::Error::<Test>::DeniedOperation.into()
			);
		}

		// When claim have already been made
		{
			// Insert in snapshot map that this claim is already made
			crate::IceSnapshotMap::<Test>::insert(
				&receiver,
				types::SnapshotInfo {
					claim_status: true,
					..Default::default()
				},
			);

			let root_origin = mock::Origin::root();
			let fail_with_already_claimed = AirdropModule::complete_transfer(
				root_origin,
				receiver.clone(),
				server_response.clone(),
			);
			assert_err!(fail_with_already_claimed, Error::<Test>::ClaimAlreadyMade);
		}
	});
}

#[test]
fn claim_request_valid() {
	mock::new_test_ext().execute_with(|| {
		let message = b"dummy-test-text";
		let icon_signature = message;
		let icon_address = message;
		let ice_address = types::AccountIdOf::<Test>::default();

		let claim_res = AirdropModule::claim_request(
			mock::Origin::signed(ice_address.clone()),
			icon_address.to_vec(),
			message.to_vec(),
			icon_signature.to_vec(),
		);

		// Make sure this request passes
		assert_ok!(claim_res);

		// Expected snapshot to be stored in map
		let expected_snapshot = types::SnapshotInfo::<Test> {
			icon_address: icon_address.to_vec(),
			amount: 0,
			defi_user: false,
			vesting_percentage: 0,
			claim_status: false,
		};

		// Make sure that the ice->snapshot map is set accordingly
		let map_data = AirdropModule::get_ice_snapshot_map(&ice_address);
		assert_eq!(map_data, Some(expected_snapshot));

		// Make sure that queue storage is populated accordingly
		let queue_data = AirdropModule::get_pending_claims(&ice_address);
		assert_eq!(queue_data, Some(()));
	});
}

#[test]
fn make_signed_call_valid() {
	let (mut test_ext, state) = new_offchain_test_ext!();

	test_ext.execute_with(|| {
		let call = pallet_airdrop::pallet::Call::sample_call { arg: 10 };
		let call_res = AirdropModule::make_signed_call(&call);

		assert_ok!(call_res);
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
