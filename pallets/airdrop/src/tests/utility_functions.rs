use super::prelude::*;

#[test]
fn update_server_account() {
	minimal_test_ext().execute_with(|| {
		assert_noop!(
			AirdropModule::set_airdrop_server_account(Origin::none(), samples::ACCOUNT_ID[1]),
			PalletError::DeniedOperation
		);

		assert_noop!(
			AirdropModule::set_airdrop_server_account(
				Origin::signed(samples::ACCOUNT_ID[1]),
				samples::ACCOUNT_ID[2]
			),
			PalletError::DeniedOperation
		);

		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			samples::ACCOUNT_ID[1]
		));
		assert_eq!(
			Some(samples::ACCOUNT_ID[1]),
			AirdropModule::get_airdrop_server_account()
		);
	});
}

#[test]
fn ensure_root_or_server() {
	minimal_test_ext().execute_with(|| {
		use sp_runtime::DispatchError::BadOrigin;

		// Set server account to be ACCOUNT_ID[0]
		let server_account = samples::ACCOUNT_ID[0];
		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			server_account.clone()
		));

		// root origin should pass
		assert_ok!(AirdropModule::ensure_root_or_server(Origin::root()));

		// Any signed other than server account should fail
		assert_err!(
			AirdropModule::ensure_root_or_server(Origin::signed(samples::ACCOUNT_ID[2])),
			BadOrigin
		);

		// Unsigned origin should fail
		assert_err!(
			AirdropModule::ensure_root_or_server(Origin::none()),
			BadOrigin
		);

		// Signed with server account should pass
		assert_ok!(AirdropModule::ensure_root_or_server(Origin::signed(
			server_account
		)));
	});
}

#[test]
fn get_vesting_amounts_split() {
	use sp_runtime::ArithmeticError;
	let expected_defi_instant_per = 40;
	let expected_non_defi_instant_per = 30;
	minimal_test_ext().execute_with(|| {
		let get_split_amounts: _ = utils::get_split_amounts::<Test>;
		let defi_instant = utils::get_instant_percentage::<Test>(true);
		let non_defi_instant = utils::get_instant_percentage::<Test>(false);

		assert_eq!(
			(expected_defi_instant_per, expected_non_defi_instant_per),
			(defi_instant, non_defi_instant),
		);
		assert_err!(
			get_split_amounts(types::ServerBalance::max_value(), defi_instant),
			ArithmeticError::Overflow
		);
		assert_eq!(Ok((10_u32.into(), 0u32.into())), get_split_amounts(10, 100));
		assert_eq!(Ok((0u32.into(), 10u32.into())), get_split_amounts(10, 0));
		assert_eq!(Ok((0u32.into(), 0u32.into())), get_split_amounts(0, 50));
		assert_eq!(Ok((0u32.into(), 0u32.into())), get_split_amounts(0, 0));

		assert_eq!(
			Ok((900_u32.into(), 2100_u32.into())),
			get_split_amounts(3_000_u32.into(), non_defi_instant)
		);

		assert_eq!(
			Ok((1200_u32.into(), 1800_u32.into())),
			get_split_amounts(3_000_u32.into(), defi_instant)
		);
		assert_eq!(
			Ok((2932538_u32.into(), 6842591_u32.into())),
			get_split_amounts(9775129_u32.into(), non_defi_instant)
		);
		assert_eq!(
			Ok((3910051_u32.into(), 5865078_u32.into())),
			get_split_amounts(9775129_u32.into(), defi_instant)
		);
	});
}

#[test]
fn get_vesting_amounts_split_no_vesting() {
	minimal_test_ext().execute_with(|| {
		let get_split_amounts: _ = utils::get_split_amounts::<Test>;
		let defi_instant = 100;
		let non_defi_instant = 100;
		assert_eq!(Ok((10_u32.into(), 0u32.into())), get_split_amounts(10, 100));
		assert_eq!(Ok((0u32.into(), 10u32.into())), get_split_amounts(10, 0));
		assert_eq!(Ok((0u32.into(), 0u32.into())), get_split_amounts(0, 50));
		assert_eq!(Ok((0u32.into(), 0u32.into())), get_split_amounts(0, 0));
		assert_eq!(
			Ok((3000u32.into(), 0u32.into())),
			get_split_amounts(3000, non_defi_instant)
		);
		assert_eq!(
			Ok((3000u32.into(), 0u32.into())),
			get_split_amounts(3000, defi_instant)
		);
		assert_eq!(
			Ok((9775129_u32.into(), 0u32.into())),
			get_split_amounts(9775129, non_defi_instant)
		);
		assert_eq!(
			Ok((9775129_u32.into(), 0u32.into())),
			get_split_amounts(9775129, defi_instant)
		);
	});
}

#[test]
fn cook_vesting_schedule() {
	type BlockToBalance = <Test as pallet_vesting::Config>::BlockNumberToBalance;
	minimal_test_ext().execute_with(|| {
		{
			let (schedule, remainder) =
				utils::new_vesting_with_deadline::<Test, 0u32>(1000u32.into(), 1000u32.into());

			let schedule = schedule.unwrap();
			assert_eq!(remainder, 0u32.into());

			assert_eq!(schedule.locked(), 1000u32.into());
			assert_eq!(schedule.per_block(), 1u32.into());
			assert_eq!(
				schedule.ending_block_as_balance::<BlockToBalance>(),
				1000u32.into()
			);
		}

		{
			let min_vesting_amount = <Test as pallet_vesting::Config>::MinVestedTransfer::get();
			let amount = min_vesting_amount - 1;
			let (schedule, remainder) = utils::new_vesting_with_deadline::<Test, 0u32>(amount, 1);

			assert_eq!(schedule, None);
			assert_eq!(remainder, amount);
		}

		{
			let (schedule, remainder) =
				utils::new_vesting_with_deadline::<Test, 0u32>(5u32.into(), 10u32.into());

			assert_eq!(None, schedule);
			assert_eq!(remainder, 5u32.into());
		}

		{
			let (schedule, remained) =
				utils::new_vesting_with_deadline::<Test, 5u32>(1002u32.into(), 10u32.into());

			let primary = schedule.unwrap();
			assert_eq!(remained, 2u32.into());

			assert_eq!(primary.locked(), 1000u32.into());
			assert_eq!(primary.per_block(), 200u32.into());
			assert_eq!(
				primary.ending_block_as_balance::<BlockToBalance>(),
				10u32.into()
			);
		}

		{
			let (schedule, remained) =
				utils::new_vesting_with_deadline::<Test, 0u32>(1006_u32.into(), 10u32.into());

			let schedule = schedule.unwrap();
			assert_eq!(remained, 6u32.into());

			assert_eq!(schedule.locked(), 1000_u32.into());
			assert_eq!(schedule.per_block(), 100_u32.into());
			assert_eq!(
				schedule.ending_block_as_balance::<BlockToBalance>(),
				10u32.into()
			);
		}

		{
			let (schedule, remained) =
				utils::new_vesting_with_deadline::<Test, 0u32>(3336553u32.into(), 10_000u32.into());

			let schedule = schedule.unwrap();
			assert_eq!(remained, 6553u32.into());

			assert_eq!(schedule.locked(), 3330000u32.into());
			assert_eq!(schedule.per_block(), 333u32.into());
			assert_eq!(
				schedule.ending_block_as_balance::<BlockToBalance>(),
				10_000u32.into()
			);
		}
	});
}

#[test]
fn making_vesting_transfer() {
	let get_per: _ = utils::get_instant_percentage::<Test>;

	minimal_test_ext().execute_with(|| {
		run_to_block(3);
		let defi_user = true;
		let amount: types::BalanceOf<Test> = 9775129_u64.into();
		let icon_address = samples::ICON_ADDRESS[0];
		type Currency = <Test as pallet_airdrop::Config>::Currency;
		// Fund creditor
		set_creditor_balance(u64::MAX);

		{
			let claimer = samples::ACCOUNT_ID[1];
			let mut snapshot = types::SnapshotInfo::<Test> {
				done_instant: false,
				done_vesting: false,
				ice_address: claimer.clone().into(),
				amount,
				defi_user,
				..Default::default()
			};

			assert_ok!(AirdropModule::do_transfer(&mut snapshot, &icon_address));

			// Ensure all amount is being transferred
			assert_eq!(9775129_u128, Currency::free_balance(&claimer));

			// Make sure user is getting at least of instant amount
			// might get more due to vesting remainder
			assert!(Currency::usable_balance(&claimer) >= 2932538_u32.into());

			// Make sure flags are updated
			assert!(snapshot.done_instant);
			assert!(snapshot.done_vesting);
		}

		// When instant transfer is true but not vesting
		{
			let claimer = samples::ACCOUNT_ID[2];
			let mut snapshot = types::SnapshotInfo::<Test> {
				done_instant: true,
				done_vesting: false,
				ice_address: claimer.clone().into(),
				amount,
				defi_user,
				..Default::default()
			};

			assert_ok!(AirdropModule::do_transfer(&mut snapshot, &icon_address));

			// Ensure amount only accounting to vesting is transferred

			let expected_transfer = {
				let vesting_amount: types::VestingBalanceOf<Test> =
					utils::get_split_amounts::<Test>(amount, get_per(defi_user))
						.unwrap()
						.1;
				let schedule = utils::new_vesting_with_deadline::<Test, 1u32>(
					vesting_amount,
					5256000u32.into(),
				)
				.0
				.unwrap();

				schedule.locked()
			};
			let user_balance = Currency::free_balance(&claimer);
			assert_eq!(expected_transfer, user_balance);

			// Ensure flag is updated
			assert!(snapshot.done_vesting && snapshot.done_instant);
		}

		// When vesting is true but not done_instant
		{
			let claimer = samples::ACCOUNT_ID[3];
			let mut snapshot = types::SnapshotInfo::<Test> {
				done_instant: false,
				done_vesting: true,
				ice_address: claimer.clone().into(),
				amount,
				defi_user,
				..Default::default()
			};

			assert_ok!(AirdropModule::do_transfer(&mut snapshot, &icon_address));

			// Ensure amount only accounting to instant is transferred
			let expected_transfer = {
				let (instant_amount, vesting_amount) =
					utils::get_split_amounts::<Test>(amount, get_per(defi_user)).unwrap();
				let remainder = utils::new_vesting_with_deadline::<Test, 1u32>(
					vesting_amount,
					5256000u32.into(),
				)
				.1;

				instant_amount + remainder
			};
			let user_balance = Currency::free_balance(&claimer);

			assert_eq!(expected_transfer, user_balance);

			// Ensure flag is updated
			assert!(snapshot.done_vesting && snapshot.done_instant);
		}

		// When both are false
		// NOTE: such snapshot will not be passed in actual from complete_transfer
		{
			let claimer = samples::ACCOUNT_ID[4];
			let mut snapshot = types::SnapshotInfo::<Test> {
				done_instant: true,
				done_vesting: true,
				ice_address: claimer.clone().into(),
				amount,
				defi_user,
				..Default::default()
			};

			assert_ok!(AirdropModule::do_transfer(&mut snapshot, &icon_address));

			// Ensure amount only accounting to instant is transferred
			assert_eq!(0_u128, Currency::free_balance(&claimer));

			// Ensure flag is updates
			assert!(snapshot.done_vesting && snapshot.done_instant);
		}
	});
}

#[test]
fn test_extract_address() {
	let payload = "icx_sendTransaction.data.{method.transfer.params.{wallet.b6e7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48}}.dataType.call.from.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.version.0x3".as_bytes();
	let expected_address =
		hex_literal::hex!("b6e7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48");
	let extracted_address = utils::extract_ice_address(payload, &expected_address).unwrap();
	assert_eq!(extracted_address, expected_address);
}

#[test]
fn respect_airdrop_state() {
	// First verify that initially everything is allowed
	assert_eq!(
		types::AirdropState::default(),
		types::AirdropState {
			block_claim_request: false,
			block_exchange_request: false,
		}
	);

	// Tests types::AirdropState::block_claim_request
	minimal_test_ext().execute_with(|| {
		// By default we shall allow it
		assert_ok!(AirdropModule::ensure_user_claim_switch());

		// Set the state to block incoming request
		assert_ok!(AirdropModule::update_airdrop_state(
			Origin::root(),
			types::AirdropState {
				block_exchange_request: false,
				block_claim_request: true,
			}
		));

		// Call the helper function
		assert_err!(
			AirdropModule::ensure_user_claim_switch(),
			PalletError::NewClaimRequestBlocked
		);

		// Call the actual dispatchable
		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::root(),
				Default::default(),
				Default::default(),
				[0; 289],
				[0; 65],
				[0; 64],
				Default::default(),
				Default::default(),
				Default::default(),
			),
			PalletError::NewClaimRequestBlocked,
		);
	});

	// Tests types::AirdropState::block_exchange_request
	minimal_test_ext().execute_with(|| {
		// By default we shall allow it
		assert_ok!(AirdropModule::ensure_exchange_claim_switch());

		// Set the state to block incoming request
		assert_ok!(AirdropModule::update_airdrop_state(
			Origin::root(),
			types::AirdropState {
				block_exchange_request: true,
				block_claim_request: false,
			}
		));

		// Call the helper function
		assert_err!(
			AirdropModule::ensure_exchange_claim_switch(),
			PalletError::NewExchangeRequestBlocked
		);

		// Call teh actual dispatchable
		assert_noop!(
			AirdropModule::dispatch_exchange_claim(
				Origin::root(),
				Default::default(),
				Default::default(),
				Default::default(),
				Default::default(),
				Default::default(),
			),
			PalletError::NewExchangeRequestBlocked
		);
	})
}

#[test]
fn validate_creditor_fund() {
	use frame_support::traits::Currency;

	minimal_test_ext().execute_with(|| {
		let existential_balance = <Test as pallet_airdrop::Config>::Currency::minimum_balance();
		let donor = samples::ACCOUNT_ID[1];
		let _put_fund =
			<Test as pallet_airdrop::Config>::Currency::deposit_creating(&donor, u64::MAX.into());

		// When creditor balance is empty.
		{
			assert_err!(
				AirdropModule::validate_creditor_fund(10),
				PalletError::InsufficientCreditorBalance
			);
		}

		// When creditor balance is exactly same as existential balance
		{
			transfer_to_creditor(&donor, existential_balance);
			assert_err!(
				AirdropModule::validate_creditor_fund(existential_balance.try_into().unwrap()),
				PalletError::InsufficientCreditorBalance,
			);
		}

		// When all of creditor balance is required
		{
			transfer_to_creditor(&donor, u32::MAX.into());
			let required_balance = <Test as pallet_airdrop::Config>::Currency::free_balance(
				&force_get_creditor_account::<Test>(),
			);

			assert_err!(
				AirdropModule::validate_creditor_fund(required_balance.try_into().unwrap()),
				PalletError::InsufficientCreditorBalance,
			);
		}

		// When only a portion of balance is required
		{
			assert_ok!(AirdropModule::validate_creditor_fund(10_000_000),);
		}
	});
}

#[test]
fn ensure_claimable_snapshot() {
	type SnapshotInfo = types::SnapshotInfo<Test>;
	minimal_test_ext().execute_with(|| {
		let both_true = SnapshotInfo {
			done_instant: true,
			done_vesting: true,
			..Default::default()
		};
		let both_false = SnapshotInfo {
			done_vesting: false,
			done_instant: false,
			..Default::default()
		};
		let instant_true_vesting_false = SnapshotInfo {
			done_instant: true,
			done_vesting: false,
			..Default::default()
		};
		let instant_false_vesting_true = SnapshotInfo {
			done_instant: false,
			done_vesting: true,
			..Default::default()
		};

		assert_ok!(AirdropModule::ensure_claimable(&both_false));
		assert_ok!(AirdropModule::ensure_claimable(&instant_true_vesting_false));
		assert_ok!(AirdropModule::ensure_claimable(&instant_false_vesting_true));
		assert_err!(
			AirdropModule::ensure_claimable(&both_true),
			PalletError::ClaimAlreadyMade,
		);
	});
}

#[test]
fn insert_or_get_snapshot() {
	use crate::{IceIconMap, IconSnapshotMap};

	// When there is previous record of this ice address
	// and new call do not match that ice_address
	minimal_test_ext().execute_with(|| {
		let ice_address = samples::ACCOUNT_ID[1];
		let icon_address_one = samples::ICON_ADDRESS[0];
		let icon_address_two = samples::ICON_ADDRESS[1];
		assert_ne!(icon_address_one, icon_address_two);

		<IceIconMap<Test>>::insert(&ice_address, icon_address_one);

		assert_noop!(
			AirdropModule::insert_or_get_snapshot(
				&icon_address_two,
				&ice_address.encode().try_into().unwrap(),
				false,
				0u32.into()
			),
			PalletError::IconAddressInUse,
		);
	});

	// There is previous record old this icon address
	// and new call do not match that icon_address
	minimal_test_ext().execute_with(|| {
		let icon_address = samples::ICON_ADDRESS[1];
		let ice_address_one = samples::ACCOUNT_ID[1];
		let ice_address_two = samples::ACCOUNT_ID[2];
		assert_ne!(ice_address_one, ice_address_two);

		let snapshot = types::SnapshotInfo::<Test>::default().ice_address(ice_address_one.clone());
		<IconSnapshotMap<Test>>::insert(&icon_address, snapshot);

		assert_noop!(
			AirdropModule::insert_or_get_snapshot(
				&icon_address,
				&ice_address_two.encode().try_into().unwrap(),
				false,
				0u32.into()
			),
			PalletError::IceAddressInUse,
		);
	});

	// There is previous old record of both address
	// and new call match that addresses
	minimal_test_ext().execute_with(|| {
		let icon_address = samples::ICON_ADDRESS[1];
		let ice_address = samples::ACCOUNT_ID[1];

		let snapshot = types::SnapshotInfo::<Test>::default().ice_address(ice_address.clone());
		<IconSnapshotMap<Test>>::insert(&icon_address, snapshot.clone());
		<IceIconMap<Test>>::insert(&ice_address, &icon_address);

		let expected_snapshot = snapshot;
		let call_result = AirdropModule::insert_or_get_snapshot(
			&icon_address,
			&ice_address.encode().try_into().unwrap(),
			false,
			0u32.into(),
		);

		assert_eq!(call_result.as_ref(), Ok(&expected_snapshot));
		assert_eq!(
			AirdropModule::get_icon_snapshot_map(&icon_address),
			Some(expected_snapshot)
		);
		assert_eq!(
			AirdropModule::get_ice_to_icon_map(&ice_address),
			Some(icon_address)
		);
	});

	// No previous record of both address
	minimal_test_ext().execute_with(|| {
		let icon_address = samples::ICON_ADDRESS[1];
		let ice_address = samples::ACCOUNT_ID[1];

		let expected_snapshot =
			types::SnapshotInfo::<Test>::default().ice_address(ice_address.clone());
		let call_result = AirdropModule::insert_or_get_snapshot(
			&icon_address,
			&ice_address.encode().try_into().unwrap(),
			false,
			0u32.into(),
		);

		assert_eq!(call_result.as_ref(), Ok(&expected_snapshot));
		assert_eq!(
			AirdropModule::get_icon_snapshot_map(&icon_address),
			Some(expected_snapshot)
		);
		assert_eq!(
			AirdropModule::get_ice_to_icon_map(&ice_address),
			Some(icon_address)
		);
	});
}

#[test]
fn storage_version() {
	minimal_test_ext().execute_with(|| {
		let version = AirdropModule::get_storage_version();
		assert_eq!(1_u32, version);
	});
}
