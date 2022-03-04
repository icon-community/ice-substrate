use super::prelude::*;

#[test]
fn pool_dispatchable_from_offchain() {
	let (mut test_ext, _, pool_state) = offchain_test_ext();

	// Basic test that single call can be put on pool
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: vec![],
				message: vec![],
				icon_signature: vec![],
			}
		));

		todo!("Verify in pool too..");
	});

	// Test that multiple call be put on pool
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: types::IconAddress::default(),
				message: vec![],
				icon_signature: vec![],
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::register_failed_claim {
				block_number: 1_u32.into(),
				icon_address: types::IconAddress::default()
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: types::IconAddress::default(),
				message: vec![],
				icon_signature: vec![],
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::donate_to_creditor {
				amount: 100_u32.into(),
				allow_death: true
			}
		));

		todo!("Verify in pool too..");
	});
}

#[test]
fn ensure_root_or_sudo() {
	minimal_test_ext().execute_with(|| {
		use sp_runtime::DispatchError::BadOrigin;

		let sudo_origin = Origin::signed(AirdropModule::get_sudo_account());
		let signed_origin = Origin::signed(not_airdrop_sudo(samples::ACCOUNT_ID[2]));
		let root_origin = Origin::root();
		let unsigned_origin = Origin::none();

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
fn making_correct_http_request() {
	let icon_address = samples::ICON_ADDRESS[0];

	let (mut test_ext, offchain_state, _) = offchain_test_ext();
	put_response(
		&mut offchain_state.write(),
		&icon_address.as_bytes().to_vec(),
		&serde_json::to_string(&samples::SERVER_DATA[0]).unwrap(),
	);

	test_ext.execute_with(|| {
		let icon_address = bytes::from_hex(icon_address).unwrap();
		let fetch_res = AirdropModule::fetch_from_server(&icon_address);
		assert_ok!(fetch_res);
	});
}

#[test]
fn failed_entry_regestration() {
	minimal_test_ext().execute_with(|| {
		let bl_num: types::BlockNumberOf<Test> = 2_u32.into();
		let claimer = bytes::from_hex(samples::ICON_ADDRESS[0]).unwrap();
		let retry = 2_u8;
		let running_bl_num = bl_num + 6;

		// Simulate we running in block running_bl_num;
		mock::System::set_block_number(running_bl_num);

		// Be sure access is controlled
		{
			assert_storage_noop!(assert_eq! {
				AirdropModule::register_failed_claim(
					Origin::signed(not_airdrop_sudo(samples::ACCOUNT_ID[1])),
					bl_num.into(),
					claimer.clone(),
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});

			assert_storage_noop!(assert_eq! {
				AirdropModule::register_failed_claim(
					Origin::none(),
					bl_num.into(),
					claimer.clone(),
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});

			assert_storage_noop!(assert_ne! {
				AirdropModule::register_failed_claim(
					Origin::signed(AirdropModule::get_sudo_account()),
					bl_num.into(),
					claimer.clone(),
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// When there is no data in map
		{
			assert_noop!(
				AirdropModule::register_failed_claim(Origin::root(), bl_num, claimer.clone()),
				PalletError::IncompleteData
			);
		}

		// Insert sample data in map
		pallet_airdrop::IceSnapshotMap::insert(&claimer, types::SnapshotInfo::<Test>::default());

		// When there is something in map but not in queue
		{
			assert_noop!(
				AirdropModule::register_failed_claim(Origin::root(), bl_num, claimer.clone()),
				PalletError::NotInQueue
			);
		}

		// Insert a sample data in queue with 0 retry remaining
		pallet_airdrop::PendingClaims::<Test>::insert(bl_num, &claimer, 0_u8);

		// When there are no more retry left in this entry
		{
			assert_err!(
				AirdropModule::register_failed_claim(Origin::root(), bl_num, claimer.clone()),
				PalletError::RetryExceed
			);
			// Still entry should be removed from queue
			assert_eq!(None, AirdropModule::get_pending_claims(bl_num, &claimer));
		}

		// Reinsert in queue with some retry count left
		pallet_airdrop::PendingClaims::<Test>::insert(bl_num, &claimer, retry);

		// This should now succeed
		{
			assert_ok!(AirdropModule::register_failed_claim(
				Origin::root(),
				bl_num,
				claimer.clone()
			));

			// Make sure entry is no longer in old key
			assert_eq!(None, AirdropModule::get_pending_claims(bl_num, &claimer));

			// Make sure entry is shifter to another key with retry decremented
			assert_eq!(
				Some(retry - 1),
				AirdropModule::get_pending_claims(running_bl_num + 1, &claimer)
			);
		}
	});
}

#[test]
fn pending_claims_getter() {
	type PendingClaimsOf = types::PendingClaimsOf<Test>;
	use samples::ICON_ADDRESS;

	let get_flattened_vec = |mut walker: PendingClaimsOf| {
		let mut res: Vec<(types::BlockNumberOf<Test>, types::IconAddress)> = vec![];

		while let Some((bl_num, mut inner_walker)) = walker.next() {
			while let Some(entry) = inner_walker.next() {
				res.push((bl_num, entry));
			}
		}

		res
	};

	let sample_entries: &[(types::BlockNumberOf<Test>, types::IconAddress)] = &[
		(1_u32.into(), bytes::from_hex(ICON_ADDRESS[1]).unwrap()),
		(1_u32.into(), bytes::from_hex(ICON_ADDRESS[0]).unwrap()),
		(2_u32.into(), bytes::from_hex(ICON_ADDRESS[3]).unwrap()),
		(10_u32.into(), bytes::from_hex(ICON_ADDRESS[2]).unwrap()),
	];

	const EMPTY: [(types::BlockNumberOf<Test>, types::IconAddress); 0] = [];

	minimal_test_ext().execute_with(|| {
		// When there is nothing in storage it should always return empty entry
		{
			let entries = get_flattened_vec(PendingClaimsOf::new(1_u32.into()..5_u32.into()));
			assert_eq!(EMPTY.to_vec(), entries);
		}

		// Make some data entry with dummy retry count
		for (k1, k2) in sample_entries {
			pallet_airdrop::PendingClaims::<Test>::insert(k1, k2, 1_u8);
		}

		// Make sure range is treated as exclusive
		{
			let entries = get_flattened_vec(PendingClaimsOf::new(0_u32.into()..1_u32.into()));
			assert_eq!(EMPTY.to_vec(), entries);

			let entries = get_flattened_vec(PendingClaimsOf::new(10_u32.into()..10_u32.into()));
			assert_eq!(EMPTY.to_vec(), entries);

			let entries = get_flattened_vec(PendingClaimsOf::new(10_u32.into()..20_u32.into()));
			assert_eq!(
				vec![(10_u32.into(), bytes::from_hex(ICON_ADDRESS[2]).unwrap())],
				entries
			);
		}

		// Make sure out of range is always empty
		{
			let entries = get_flattened_vec(PendingClaimsOf::new(20_u32.into()..30_u32.into()));
			assert_eq!(EMPTY.to_vec(), entries);
		}

		// Make sure correct data is returned
		{
			let entries = get_flattened_vec(PendingClaimsOf::new(1_u32.into()..3_u32.into()));
			assert_eq!(
				vec![
					(1_u32.into(), bytes::from_hex(ICON_ADDRESS[1]).unwrap()),
					(1_u32.into(), bytes::from_hex(ICON_ADDRESS[0]).unwrap()),
					(2_u32.into(), bytes::from_hex(ICON_ADDRESS[3]).unwrap())
				],
				entries
			);
		}
	})
}
