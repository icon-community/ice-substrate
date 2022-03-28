use super::prelude::*;
use frame_support::traits::Hooks;
use sp_runtime::DispatchError;

#[test]
fn claim_request_access() {
	minimal_test_ext().execute_with(|| {
		// Unsigned should not be able to call
		assert_noop!(
			AirdropModule::claim_request(Origin::none(), samples::ICON_ADDRESS[1], vec![], vec![]),
			DispatchError::BadOrigin
		);

		// Root should not be able to call
		assert_noop!(
			AirdropModule::claim_request(Origin::root(), samples::ICON_ADDRESS[1], vec![], vec![]),
			DispatchError::BadOrigin
		);

		// Signed user should be able to call
		assert_ok!(AirdropModule::claim_request(
			Origin::signed(samples::ACCOUNT_ID[0]),
			samples::ICON_ADDRESS[1],
			vec![],
			vec![]
		));
	});
}

#[test]
fn already_in_map() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ICON_ADDRESS[1];

		// Insert this entry to map
		pallet_airdrop::IceSnapshotMap::<Test>::insert(&claimer, types::SnapshotInfo::default());

		// Should be an error
		assert_noop!(
			AirdropModule::claim_request(
				Origin::signed(samples::ACCOUNT_ID[0]),
				claimer.clone(),
				vec![],
				vec![]
			),
			PalletError::RequestAlreadyMade
		);
	});
}

#[test]
fn valid_claim_request() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ICON_ADDRESS[1];

		assert_ok!(AirdropModule::claim_request(
			Origin::signed(samples::ACCOUNT_ID[0]),
			claimer.clone(),
			vec![],
			vec![]
		));

		let expected_snapshot = types::SnapshotInfo::<Test> {
			ice_address: samples::ACCOUNT_ID[0],
			amount: 0_u32.into(),
			defi_user: false,
			vesting_percentage: 0_u32.into(),
			claim_status: false,
		};

		// Make sure correct data is inserted in map
		assert_eq!(
			Some(expected_snapshot.clone()),
			AirdropModule::get_icon_snapshot_map(&claimer)
		);

		// Make sure correct data is inserted in queue
		let in_bl_num: types::BlockNumberOf<Test> = 0_u32.into();
		assert_eq!(
			Some(2_u8),
			AirdropModule::get_pending_claims(&in_bl_num, &claimer)
		);
	});
}

#[test]
fn fail_on_non_existent_data() {
	let (mut test_ext, offchain_state, pool_state, ocw_pub) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[0];
	put_response(
		&mut offchain_state.write(),
		&icon_address,
		r#""NonExistentData""#,
	);

	test_ext.execute_with(|| {
		let claimer = icon_address;
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();

		assert_ok!(AirdropModule::set_offchain_account(
			Origin::root(),
			ocw_pub.into_account()
		));

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)));

		assert_tx_call(
			&[&PalletCall::register_failed_claim {
				icon_address: claimer.clone(),
				block_number: bl_num,
			}],
			&pool_state.read(),
		);
	});
}

#[test]
fn remove_on_zero_ice() {
	let (mut test_ext, offchain_state, pool_state, ocw_pub) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[1];
	let mut server_response = samples::SERVER_DATA[1];
	server_response.amount = 0_u32.into();

	put_response(
		&mut offchain_state.write(),
		&icon_address,
		&serde_json::to_string(&server_response).unwrap(),
	);

	test_ext.execute_with(|| {
		let claimer = icon_address;
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();

		assert_ok!(AirdropModule::set_offchain_account(
			Origin::root(),
			ocw_pub.into_account()
		));

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)));

		assert_tx_call(
			&[&PalletCall::remove_from_pending_queue {
				block_number: bl_num.clone(),
				icon_address: claimer.clone(),
			}],
			&pool_state.read(),
		)
	});
}

#[test]
fn valid_process_claim() {
	let (mut test_ext, offchain_state, pool_state, ocw_pub) = offchain_test_ext();
	let icon_address = samples::ICON_ADDRESS[0];

	put_response(
		&mut offchain_state.write(),
		&icon_address,
		&serde_json::to_string(&samples::SERVER_DATA[1]).unwrap(),
	);

	test_ext.execute_with(|| {
		let claimer = icon_address.clone();
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();

		assert_ok!(AirdropModule::set_offchain_account(
			Origin::root(),
			ocw_pub.into_account()
		));

		assert_ok!(AirdropModule::process_claim_request((
			bl_num,
			claimer.clone()
		)),);

		assert_tx_call(
			&[&PalletCall::complete_transfer {
				block_number: bl_num.clone(),
				receiver_icon: claimer.clone(),
				server_response: samples::SERVER_DATA[1],
			}],
			&pool_state.read(),
		);
	});
}

#[test]
fn multi_ice_single_icon() {
	minimal_test_ext().execute_with(|| {
		let icon_address = samples::ICON_ADDRESS[0];
		let ice_address_one = samples::ACCOUNT_ID[1];
		let ice_address_two = samples::ACCOUNT_ID[2];

		// Claim with first ice address & main icon address
		{
			assert_ok!(AirdropModule::claim_request(
				Origin::signed(ice_address_one.clone()),
				icon_address.clone(),
				vec![],
				vec![]
			));
		}

		// Claim with second ice address & main icon address ( same address as above )
		{
			assert_noop!(
				AirdropModule::claim_request(
					Origin::signed(ice_address_two.clone()),
					icon_address.clone(),
					vec![],
					vec![],
				),
				PalletError::RequestAlreadyMade
			);
		}
	});
}

#[test]
fn multi_icon_single_ice() {
	minimal_test_ext().execute_with(|| {
		let icon_address_first = samples::ICON_ADDRESS[1];
		let icon_address_second = samples::ICON_ADDRESS[2];
		let ice_address = samples::ACCOUNT_ID[1];

		// claim with primary ice_address and first icon address
		{
			assert_ok!(AirdropModule::claim_request(
				Origin::signed(ice_address.clone()),
				icon_address_first,
				vec![],
				vec![]
			));
		}

		// Claim with primary ice_address ( ice_address same as above ) & second icon_address
		{
			assert_ok!(AirdropModule::claim_request(
				Origin::signed(ice_address.clone()),
				icon_address_second,
				vec![],
				vec![]
			));
		}
	});
}

#[test]
fn complete_flow() {
	let claimer_ice_address = samples::ACCOUNT_ID[1];
	let claimer_icon_address = samples::ICON_ADDRESS[1];
	let server_data = samples::SERVER_DATA[0];

	let (mut test_ext, offchain_state, pool_state, ocw_pub) = offchain_test_ext();

	put_response(
		&mut offchain_state.write(),
		&claimer_icon_address,
		&serde_json::to_string(&server_data).unwrap(),
	);

	test_ext.execute_with(|| {
		// Make sure creditor have enough balance
		assert_ok!(<Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::Origin::root(),
			AirdropModule::get_creditor_account(),
			10_000_000_000,
			10_000_u32.into(),
		));

		// Set an account as offchain authorised
		assert_ok!(AirdropModule::set_offchain_account(
			Origin::root(),
			ocw_pub.into_account()
		));

		// Get a block number where offchian worker will run
		let mut inserted_in_bl_num: types::BlockNumberOf<Test> = 10_u32.into();
		while !AirdropModule::should_run_on_this_block(inserted_in_bl_num + 2_u64) {
			inserted_in_bl_num += 2_u64;
		}
		run_to_block(inserted_in_bl_num);

		// Suppose we have done all processing 3 plock previous to current one
		let cleared_upto: types::BlockNumberOf<Test> = inserted_in_bl_num - 3_u64;
		assert_ok!(AirdropModule::update_processed_upto_counter(
			Origin::root(),
			cleared_upto
		));

		// Make a claim reqest
		assert_ok!(AirdropModule::claim_request(
			Origin::signed(claimer_ice_address.clone()),
			claimer_icon_address.clone(),
			b"any-messsage".to_vec(),
			b"any-signature".to_vec()
		));

		let current_block_number = inserted_in_bl_num + 2_u64;
		run_to_block(inserted_in_bl_num);

		// Call offchain worker to do further processing
		AirdropModule::offchain_worker(current_block_number);

		// Make sure expected call is in queue and make those call directly later on
		assert_tx_call(
			&[
				&PalletCall::complete_transfer {
					block_number: inserted_in_bl_num,
					receiver_icon: claimer_icon_address.clone(),
					server_response: server_data.clone(),
				},
				&PalletCall::update_processed_upto_counter {
					new_value: current_block_number - 1_u64,
				},
			],
			&pool_state.read(),
		);
		assert_ok!(AirdropModule::complete_transfer(
			Origin::signed(AirdropModule::get_offchain_account().unwrap()),
			inserted_in_bl_num,
			claimer_icon_address.clone(),
			server_data.clone()
		));
		assert_ok!(AirdropModule::update_processed_upto_counter(
			Origin::root(),
			current_block_number - 1_u64,
		));

		// Make sure user got right balance
		assert_eq!(
			server_data.amount + server_data.omm + server_data.stake,
			<Test as pallet_airdrop::Config>::Currency::free_balance(&claimer_ice_address),
		);

		// Make sure queue is cleared
		assert_eq!(
			None,
			AirdropModule::get_pending_claims(inserted_in_bl_num, &claimer_icon_address)
		);

		// Make sure claim_status is updated
		assert!(
			AirdropModule::get_icon_snapshot_map(&claimer_icon_address)
				.expect("Should be in map")
				.claim_status
		);

		// Make sure processed upto is updated
		assert_eq!(
			AirdropModule::get_processed_upto_counter(),
			current_block_number - 1_u64
		);
	});
}
