use super::prelude::*;
use sp_runtime::DispatchError;

#[test]
fn claim_request_access() {
	minimal_test_ext().execute_with(|| {
		// Unsigned should not be able to call
		assert_noop!(
			AirdropModule::claim_request(Origin::none(), vec![], vec![], vec![]),
			DispatchError::BadOrigin
		);

		// Root should not be able to call
		assert_noop!(
			AirdropModule::claim_request(Origin::root(), vec![], vec![], vec![]),
			DispatchError::BadOrigin
		);

		// Signed user should be able to call
		assert_ok!(AirdropModule::claim_request(
			Origin::signed(samples::ACCOUNT_ID[0]),
			vec![],
			vec![],
			vec![]
		));
	});
}

#[test]
fn already_in_map() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ACCOUNT_ID[1];

		// Insert this entry to map
		pallet_airdrop::IceSnapshotMap::<Test>::insert(&claimer, types::SnapshotInfo::default());

		// Should be an error
		assert_noop!(
			AirdropModule::claim_request(Origin::signed(claimer.clone()), vec![], vec![], vec![]),
			PalletError::RequestAlreadyMade
		);
	});
}

#[test]
fn valid_claim() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ACCOUNT_ID[1];

		assert_ok!(AirdropModule::claim_request(
			Origin::signed(claimer.clone()),
			vec![],
			vec![],
			vec![]
		));

		let expected_snapshot = types::SnapshotInfo::<Test> {
			icon_address: vec![],
			amount: 0_u32.into(),
			defi_user: false,
			vesting_percentage: 0_u32.into(),
			claim_status: false,
		};

		// Make sure correct data is inserted in map
		assert_eq!(
			Some(expected_snapshot),
			AirdropModule::get_ice_snapshot_map(&claimer)
		);

		// Make sure correct data is inserted in queue
		let in_bl_num: types::BlockNumberOf<Test> = 0_u32.into();
		assert_eq!(
			Some(2_u8),
			AirdropModule::get_pending_claims(&in_bl_num, &claimer)
		);
	});
}
