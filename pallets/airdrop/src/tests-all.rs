use crate as pallet_airdrop;
use crate::{
	mock::{self, AirdropModule, Test},
	types, Error,
};
use frame_support::{assert_err, assert_noop, assert_ok};
use sp_core::offchain::testing::OffchainState;

/// Sample icon address when sent to server retrurning defined response
const PREDEFINED_REQUEST_RESPONSE: (&str, &str) = (
	"0x000000000000000000000000000000",
	r#"{"omm":0,"balanced":300,"stake":464764,"defi_user":true}"#,
);

#[test]
fn making_http_request() {
	let (mut test_ext, state) = new_offchain_test_ext();
	put_response(&mut state.write());

	test_ext.execute_with(|| {
		let icon_address = sp_core::bytes::from_hex(PREDEFINED_REQUEST_RESPONSE.0).unwrap();
		let fetch_res = mock::AirdropModule::fetch_from_server(icon_address);
		assert_ok!(fetch_res);
	});
}

#[test]
fn process_claim_invalid() {
	let (mut test_ext, state) = new_offchain_test_ext();

	test_ext.execute_with(|| {
		let ice_address = types::AccountIdOf::<Test>::default();

		// Nothing is in queue yet so, should fail with no_icon_address
		let no_icon_address =
			AirdropModule::process_claim_request((0_u32.into(), ice_address.clone()));
		assert_eq!(
			no_icon_address.unwrap_err(),
			types::ClaimError::NoIconAddress
		);
	});
}

#[test]
fn process_claim_valid() {
	let (mut test_ext, state) = new_offchain_test_ext();

	put_response(&mut state.write());

	test_ext.execute_with(|| {
		let ice_address = types::AccountIdOf::<Test>::default();
		let icon_address = sp_core::bytes::from_hex(PREDEFINED_REQUEST_RESPONSE.0).unwrap();
		let snapshot = types::SnapshotInfo::<Test>::default();

		crate::IceSnapshotMap::insert(ice_address, snapshot.clone().icon_address(icon_address));

		let should_be_ok =
			AirdropModule::process_claim_request((1_u32.into(), ice_address.clone()));
		assert_ok!(should_be_ok);
	});
}

#[test]
fn claim_request_invalid() {
	mock::new_test_ext().execute_with(|| {
		// Called with non-signed origin
		{
			assert_noop!(
				AirdropModule::claim_request(mock::Origin::root(), vec![], vec![], vec![]),
				sp_runtime::DispatchError::BadOrigin
			);
			assert_noop!(
				AirdropModule::claim_request(mock::Origin::none(), vec![], vec![], vec![]),
				sp_runtime::DispatchError::BadOrigin
			);
		}

		// Already on map
		{
			let ice_address = types::AccountIdOf::<Test>::default();
			let claim_res = AirdropModule::claim_request(
				mock::Origin::signed(ice_address.clone()),
				b"icon-address".to_vec(),
				b"dummt-message".to_vec(),
				b"dummy-signature".to_vec(),
			);
			assert_ok!(claim_res);

			// Make sure no storage is mutated & an error is thrown
			assert_noop!(
				AirdropModule::claim_request(
					mock::Origin::signed(ice_address.clone()),
					b"icon-address".to_vec(),
					b"dummt-message".to_vec(),
					b"dummy-signature".to_vec(),
				),
				Error::<Test>::RequestAlreadyMade
			);
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
		let bl_num = AirdropModule::get_current_block_number();

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
		let queue_data = AirdropModule::get_pending_claims(&bl_num, &ice_address);
		assert_eq!(queue_data, Some(pallet_airdrop::DEFAULT_RETRY_COUNT));
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