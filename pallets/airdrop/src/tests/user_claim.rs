use super::prelude::*;
use crate::{tests::UserClaimTestCase, Config, ServerAccount};
use frame_support::traits::Currency;

#[test]
fn claim_success() {
	minimal_test_ext().execute_with(|| {
		let server_account = samples::ACCOUNT_ID[0].into_account();
		<ServerAccount<Test>>::put(server_account);
		set_creditor_balance(10_000_0000);

		let mut case = UserClaimTestCase::default();
		case.amount = 12_017_332_u64.into();

		assert_ok!(AirdropModule::dispatch_user_claim(
			Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
			case.icon_address,
			case.ice_address,
			case.message,
			case.icon_signature,
			case.ice_signature,
			case.amount,
			case.defi_user,
			case.merkle_proofs,
		));

		let ice_account = AirdropModule::convert_to_account_id(case.ice_address.clone()).unwrap();
		let total_balance = <Test as Config>::Currency::total_balance(&ice_account);
		let usable_balance = <Test as Config>::Currency::usable_balance(&ice_account);
		let snapshot = <pallet_airdrop::IconSnapshotMap<Test>>::get(&case.icon_address).unwrap();
		let mapped_icon_wallet = AirdropModule::get_ice_to_icon_map(&ice_account);

		let expected_usable_amount;
		let expected_vesting_block_number;
		let expected_icon_address = Some(case.icon_address);
		if cfg!(feature = "no-vesting") {
			expected_usable_amount = case.amount;
			expected_vesting_block_number = None;
		} else {
			expected_usable_amount = 6761333;
			expected_vesting_block_number = Some(0);
		}

		assert_eq!(total_balance, case.amount);
		assert_eq!(usable_balance, expected_usable_amount);
		assert_eq!(mapped_icon_wallet, expected_icon_address);
		assert_eq!(snapshot.vesting_block_number, expected_vesting_block_number);
		assert_eq!(snapshot.initial_transfer, usable_balance);
		assert_eq!(snapshot.instant_block_number, Some(0));
		assert_eq!(snapshot.done_instant, true);
		assert_eq!(snapshot.done_vesting, true);
	});
}

#[test]
fn insufficient_balance() {
	let ofw_account = samples::ACCOUNT_ID[0].into_account();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			ofw_account
		));

		let mut case = UserClaimTestCase::default();
		case.amount = 10017332_u64.into();
		let creditor_account = force_get_creditor_account::<Test>();
		<Test as Config>::Currency::set_balance(
			mock::Origin::root(),
			creditor_account,
			10_u32.into(),
			10_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
				case.icon_address,
				case.ice_address.clone(),
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs
			),
			PalletError::InsufficientCreditorBalance
		);
	});
}

#[test]
fn already_claimed() {
	let ofw_account = samples::ACCOUNT_ID[0].into_account();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			ofw_account
		));
		let mut case = UserClaimTestCase::default();
		let ice_account = AirdropModule::convert_to_account_id(case.ice_address).unwrap();
		case.amount = 10017332_u64.into();

		let mut snapshot = types::SnapshotInfo::default().ice_address(ice_account.clone());
		snapshot.done_instant = true;
		snapshot.done_vesting = true;

		pallet_airdrop::IconSnapshotMap::<Test>::insert(&case.icon_address, snapshot);
		let creditor_account = force_get_creditor_account::<Test>();

		<Test as Config>::Currency::set_balance(
			mock::Origin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
				case.icon_address,
				case.ice_address.clone(),
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs
			),
			PalletError::ClaimAlreadyMade
		);
	});
}

#[test]
fn invalid_payload() {
	let ofw_account = samples::ACCOUNT_ID[0].into_account();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
        assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			ofw_account
		));
		let mut case = UserClaimTestCase::default();

		case.message = *b"icx_sendTransaction.data.{method.transfer.params.{wallet.eee7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48}}.dataType.call.from.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.version.0x3";
		let creditor_account = force_get_creditor_account::<Test>();

		<Test as Config>::Currency::set_balance(
			mock::Origin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
				case.icon_address,
				case.ice_address.clone(),
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs
			),
			PalletError::InvalidMessagePayload
		);
	});
}

#[test]
fn invalid_ice_signature() {
	let ofw_account = samples::ACCOUNT_ID[0].into_account();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			ofw_account
		));
		let mut case = UserClaimTestCase::default();
		case.ice_signature = [0u8; 64];

		let creditor_account = force_get_creditor_account::<Test>();
		<Test as Config>::Currency::set_balance(
			mock::Origin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
				case.icon_address,
				case.ice_address.clone(),
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs
			),
			PalletError::InvalidIceSignature
		);
	});
}

#[test]
fn invalid_icon_signature() {
	let ofw_account = samples::ACCOUNT_ID[0].into_account();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::set_airdrop_server_account(
			Origin::root(),
			ofw_account
		));
		let mut case = UserClaimTestCase::default();
		case.icon_signature = [0u8; 65];

		let creditor_account = force_get_creditor_account::<Test>();
		<Test as Config>::Currency::set_balance(
			mock::Origin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_user_claim(
				Origin::signed(AirdropModule::get_airdrop_server_account().unwrap()),
				case.icon_address,
				case.ice_address.clone(),
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs
			),
			PalletError::InvalidSignature
		);
	});
}

#[test]
fn respect_vesting_pallet_min_transfer() {
	minimal_test_ext().execute_with(|| {
		set_creditor_balance(10_000_000);
		run_to_block(4);
		let is_defi_user = false;
		let ice_address = samples::ACCOUNT_ID[1];

		// We intentionally make it less than crate::tests::mock::MinVestingTransfer
		let total_amount = <Test as pallet_vesting::Config>::MinVestedTransfer::get() - 1;

		let mut snapshot =
			types::SnapshotInfo::<Test>::new(ice_address, is_defi_user, total_amount);

		let transfer_res = transfer::do_transfer::<Test>(&mut snapshot);

		assert_ok!(transfer_res);
		assert!(snapshot.done_vesting);
		assert!(snapshot.done_instant);
		assert_eq!(
			total_amount,
			<Test as Config>::Currency::total_balance(&ice_address)
		);

		// vesting amount derived from given total_amount will always be less
		// than pallet_vesting::Config::vestingMinTransfer in this case
		// that means everything was transferred as instant amount
		// this means snapshot.vesting_block_number should not have been set
		assert_eq!(None, snapshot.vesting_block_number);
	});
}

#[test]
fn partial_transfer_can_reclaim() {
	let vesting_period = Test::VESTING_TERMS.vesting_period;
	minimal_test_ext().execute_with(|| {
		run_to_block(1);

		let mut case = UserClaimTestCase::default();
		case.amount = 10_u64.pow(18).into();
		let ice_account = AirdropModule::convert_to_account_id(case.ice_address.clone()).unwrap();
		set_creditor_balance(Bounded::max_value());

		let mut user_balance = <Test as Config>::Currency::total_balance(&ice_account);
		assert_eq!(user_balance, 0u32.into());

		let (init_instant_amount, init_vesting_amount) = utils::get_split_amounts::<Test>(
			case.amount,
			utils::get_instant_percentage::<Test>(case.defi_user),
		)
		.unwrap();
		let (vesting_schedule, reminding_amount) = utils::new_vesting_with_deadline::<
			Test,
			{ transfer::VESTING_APPLICABLE_FROM },
		>(init_vesting_amount, vesting_period.into());
		let vesting_amount = vesting_schedule.map(|s| s.locked()).unwrap_or(0u32.into());
		let instant_amount = init_instant_amount + reminding_amount;

		// Eat all vesting slots so next vesting will fail
		{
			let vesting_count_limit = <Test as pallet_vesting::Config>::MAX_VESTING_SCHEDULES;
			let mut amount_consumed = 0;
			for i in 0..vesting_count_limit {
				let res = pallet_vesting::Pallet::<Test>::vested_transfer(
					Origin::signed(force_get_creditor_account::<Test>()),
					ice_account.clone(),
					types::VestingInfoOf::<Test>::new(10_000, 2000, 5),
				)
				.map(|_| i);
				assert_eq!(res, Ok(i));
				amount_consumed += 10_000;
			}

			let new_balance = <Test as Config>::Currency::total_balance(&ice_account);
			assert_eq!(new_balance, user_balance + amount_consumed);
			user_balance = new_balance;
		}

		// Try to claim in first attempt.
		// It will only let instant transfer to pass
		{
			let case = case.clone();
			assert_ok!(AirdropModule::dispatch_user_claim(
				Origin::root(),
				case.icon_address,
				case.ice_address,
				case.message,
				case.icon_signature,
				case.ice_signature,
				case.amount,
				case.defi_user,
				case.merkle_proofs,
			));

			let snapshot = AirdropModule::get_icon_snapshot_map(&case.icon_address).unwrap();
			let mapped_icon_wallet = AirdropModule::get_ice_to_icon_map(&ice_account);
			let new_balance = <Test as Config>::Currency::total_balance(&ice_account);
			assert!(snapshot.done_instant);
			assert_eq!(mapped_icon_wallet.as_ref(), Some(&case.icon_address));
			assert_eq!(new_balance, user_balance + instant_amount);

			let expected_vesting_status = cfg!(feature = "no-vesting");
			assert_eq!(snapshot.done_vesting, expected_vesting_status);

			user_balance = new_balance;
		}

		// Release the streams vesting schedules put previously
		{
			run_to_block(12);
			assert_ok!(pallet_vesting::Pallet::<Test>::vest(Origin::signed(
				ice_account.clone()
			)));
		}

		let reclaim_res = AirdropModule::dispatch_user_claim(
			Origin::root(),
			case.icon_address,
			case.ice_address,
			case.message,
			case.icon_signature,
			case.ice_signature,
			case.amount,
			case.defi_user,
			case.merkle_proofs,
		)
		.map(|_| ());
		let expected_res;
		let expected_vesting_block_number;
		let expected_final_balance = user_balance + vesting_amount;
		let expected_instant_block_number = Some(1);
		if cfg!(feature = "no-vesting") {
			expected_res = Err(PalletError::ClaimAlreadyMade.into());
			expected_vesting_block_number = None;
		} else {
			expected_res = Ok(());
			expected_vesting_block_number = Some(12);
		};

		let snapshot = AirdropModule::get_icon_snapshot_map(&case.icon_address).unwrap();
		let mapped_icon_wallet = AirdropModule::get_ice_to_icon_map(&ice_account);
		let final_balance = <Test as Config>::Currency::total_balance(&ice_account);
		assert_eq!(reclaim_res, expected_res);
		assert!(snapshot.done_instant);
		assert!(snapshot.done_vesting);
		assert_eq!(mapped_icon_wallet.as_ref(), Some(&case.icon_address));
		assert_eq!(expected_instant_block_number, snapshot.instant_block_number);
		assert_eq!(expected_vesting_block_number, snapshot.vesting_block_number);
		assert_eq!(expected_final_balance, final_balance);
	});
}
