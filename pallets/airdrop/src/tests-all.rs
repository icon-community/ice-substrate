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