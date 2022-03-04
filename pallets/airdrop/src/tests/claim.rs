use super::prelude::*;
use frame_support::traits::Hooks;
use sp_runtime::DispatchError;
use types::ClaimError;

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
fn valid_claim_request() {
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

		// Make sure proper event is emitted
		assert_eq!(
			get_last_event().expect("Event ClaimRequestSucceed expected"),
			PalletEvent::ClaimRequestSucced(
				types::BlockNumberOf::<Test>::default(),
				claimer.clone()
			).into(),
		);
	});
}

#[test]
fn ocw_process_invalid_entry() {
	let (mut test_ext, _state) = offchain_test_ext();

	test_ext.execute_with(|| {
		let claimer = samples::ACCOUNT_ID[0];
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();

		// When given entry is not in queue
		{
			assert_err!(
				AirdropModule::process_claim_request((bl_num, claimer.clone())),
				ClaimError::NoIconAddress
			);
		}
	});
}

#[test]
fn server_return_valid_error() {
	use types::ServerError;
	let icon_address = samples::ICON_ADDRESS[0];
	let (mut test_ext, state) = offchain_test_ext();
	put_response(
		&mut state.write(),
		&icon_address.as_bytes().to_vec(),
		r#"NonExistentData"#,
	);

	test_ext.execute_with(|| {
		assert_err!(
			AirdropModule::fetch_from_server(sp_core::bytes::from_hex(icon_address).unwrap()),
			ClaimError::ServerError(ServerError::NonExistentData),
		);
	});
}

#[test]
fn fail_on_non_existent_data() {
	let (mut test_ext, state) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[0];
	put_response(
		&mut state.write(),
		&icon_address.as_bytes().to_vec(),
		r#"NonExistentData"#,
	);

	test_ext.execute_with(|| {
		let claimer = samples::ACCOUNT_ID[0];
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();
		let snapshot = types::SnapshotInfo::<Test>::default()
			.icon_address(bytes::from_hex(icon_address).unwrap())
			.clone();

		// Insert in map
		pallet_airdrop::IceSnapshotMap::insert(&claimer, &snapshot);

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)),);

		todo!("Check the pool that proper call is placed");
	});
}

#[test]
fn remove_on_zero_ice() {
	let (mut test_ext, state) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[0];
	let mut server_response = samples::SERVER_DATA[1];
	server_response.amount = 0_u32.into();

	put_response(
		&mut state.write(),
		&icon_address.as_bytes().to_vec(),
		&serde_json::to_string(&server_response).unwrap(),
	);

	test_ext.execute_with(|| {
		let claimer = samples::ACCOUNT_ID[1];
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();
		let snapshot = types::SnapshotInfo::<Test>::default()
			.icon_address(bytes::from_hex(icon_address).unwrap())
			.clone();

		// Insert in map
		pallet_airdrop::IceSnapshotMap::insert(&claimer, &snapshot);

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)),);

		todo!("Check the pool that proper call is placed");
	});
}

#[test]
fn valid_process_claim() {
	let (mut test_ext, state) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[0];

	put_response(
		&mut state.write(),
		&icon_address.as_bytes().to_vec(),
		&serde_json::to_string(&samples::SERVER_DATA[1]).unwrap(),
	);

	test_ext.execute_with(|| {
		let claimer = samples::ACCOUNT_ID[1];
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();
		let snapshot = types::SnapshotInfo::<Test>::default()
			.icon_address(bytes::from_hex(icon_address).unwrap())
			.clone();

		// Insert in map
		pallet_airdrop::IceSnapshotMap::insert(&claimer, &snapshot);

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)),);

		todo!("Check the pool that proper call is placed");
	});
}

#[test]
fn complete_flow() {
	let claimer_ice_address = samples::ACCOUNT_ID[1];
	let claimer_icon_address = samples::ICON_ADDRESS[1];
	let server_data = samples::SERVER_DATA[0];

	minimal_test_ext().execute_with(|| {
		// Get a block number where offchian worker will run
		let mut current_block_number: types::BlockNumberOf<Test> = 10_u32.into();
		while !AirdropModule::should_run_on_this_block(current_block_number) {
			current_block_number += types::BlockNumberOf::<Test>::from(1_u32);
		}

		// Set the current block number to certain height
		mock::System::set_block_number(current_block_number);
		// Suppose we have done all processing 3 plock previous to current one
		let cleared_upto: types::BlockNumberOf<Test> =
			current_block_number - types::BlockNumberOf::<Test>::from(3_u32);

		// Make sure creditor have enough balance
		assert_ok!(<Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::Origin::root(),
			AirdropModule::get_creditor_account(),
			10_00_000_u32.into(),
			10_000_u32.into(),
		));

		// Make a claim reqest
		assert_ok!(AirdropModule::claim_request(
			Origin::signed(claimer_ice_address.clone()),
			bytes::from_hex(claimer_icon_address.clone()).unwrap(),
			b"any-messsage".to_vec(),
			b"any-signature".to_vec()
		));

		// Then process that claim request
		AirdropModule::offchain_worker(current_block_number);

		todo!("Make sure complete_transfer call is in transaction pool");

		// Make sure user got right balance
		assert_eq!(
			server_data.amount,
			<Test as pallet_airdrop::Config>::Currency::free_balance(&claimer_ice_address),
		);

		// Make sure queue is cleared
		assert_eq!(
			None,
			AirdropModule::get_pending_claims(current_block_number, &claimer_ice_address)
		);

		// Make sure claim_status is updated
		assert!(
			AirdropModule::get_ice_snapshot_map(&claimer_ice_address)
				.expect("Should be in map")
				.claim_status
		);
	});
}
