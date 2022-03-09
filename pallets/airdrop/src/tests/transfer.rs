use super::prelude::*;
type BalanceError = pallet_balances::pallet::Error<Test>;

#[test]
fn complete_transfer_access() {
	// All the calls must fail anyway but the failing error should
	// not be anything related to access and storage should be kept unmodified

	minimal_test_ext().execute_with(|| {
		// Make sure root can call this
		{
			assert_storage_noop!(assert_ne! {
				AirdropModule::complete_transfer(
					Origin::root(),
					1_u32.into(),
					samples::ICON_ADDRESS[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// Make sure sudo can call this
		{
			assert_ok!(AirdropModule::set_offchain_account(
				Origin::root(),
				samples::ACCOUNT_ID[1]
			));
			assert_storage_noop!(assert_ne! {
				AirdropModule::complete_transfer(
					Origin::signed(AirdropModule::get_offchain_account().unwrap()),
					1_u32.into(),
					samples::ICON_ADDRESS[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// Make sure other signed user can't called this
		{
			let non_sudo = not_offchain_account(samples::ACCOUNT_ID[2]);
			assert_storage_noop!(assert_eq! {
				AirdropModule::complete_transfer(
					Origin::signed(non_sudo),
					1_u32.into(),
					samples::ICON_ADDRESS[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// Unsigned user must also not be able to call this
		{
			assert_storage_noop!(assert_eq! {
				AirdropModule::complete_transfer(
					Origin::none(),
					1_u32.into(),
					samples::ICON_ADDRESS[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}
	});
}

#[test]
fn no_data_in_map() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ICON_ADDRESS[0];
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();

		// Insert some data in queue
		pallet_airdrop::PendingClaims::<Test>::insert(bl_num, &claimer, 1_u8);
		// also fund creditor
		credit_creditor(100_000_u32);

		assert_noop!(
			AirdropModule::complete_transfer(
				Origin::root(),
				1_u32.into(),
				claimer,
				samples::SERVER_DATA[0]
			),
			PalletError::IncompleteData
		);
	});
}

#[test]
fn no_data_in_queue() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ICON_ADDRESS[0];

		// Insert some data in map
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&claimer,
			types::SnapshotInfo::<Test>::default(),
		);
		// also fund creditor
		credit_creditor(100_000_u32);

		assert_noop!(
			AirdropModule::complete_transfer(
				Origin::root(),
				1_u32.into(),
				claimer,
				samples::SERVER_DATA[0]
			),
			PalletError::NotInQueue
		);
	});
}

#[test]
fn already_claimed() {
	minimal_test_ext().execute_with(|| {
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();
		let receiver = samples::ICON_ADDRESS[0];

		// Insert in queue
		pallet_airdrop::PendingClaims::<Test>::insert(bl_num, &receiver, 1_u8);
		// Insert in snapshot map that this claim is already made
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&receiver,
			types::SnapshotInfo {
				claim_status: true,
				..Default::default()
			},
		);

		assert_noop!(
			AirdropModule::complete_transfer(
				mock::Origin::root(),
				bl_num,
				receiver,
				samples::SERVER_DATA[0],
			),
			PalletError::ClaimAlreadyMade
		);
	});
}

#[test]
fn insufficient_creditor_balance() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ICON_ADDRESS[0];
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();
		let new_bl_num: types::BlockNumberOf<Test> = 6_u32.into();
		let initial_retry = 1;

		// Insert some data in map and queue
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&claimer,
			types::SnapshotInfo::<Test>::default(),
		);
		// claim was requested in block height 1
		pallet_airdrop::PendingClaims::<Test>::insert(bl_num, &claimer, initial_retry);

		// Pseudo-simulate that now block number is new_bl_num
		// Note: behaviour like on_finalize is not called from here
		tests::System::set_block_number(new_bl_num);

		assert_err!(
			AirdropModule::complete_transfer(
				Origin::root(),
				bl_num,
				claimer.clone(),
				samples::SERVER_DATA[0]
			),
			BalanceError::InsufficientBalance
		);

		// Behavioural expectation on this type of failure
		{
			// Make sure entry is removed from prevous block
			assert_eq!(None, AirdropModule::get_pending_claims(bl_num, &claimer));

			// Must be in another block ( current + 1 ) with decremented retry
			let new_retry = AirdropModule::get_pending_claims(
				new_bl_num.saturating_add(1_u32.into()),
				&claimer,
			);
			assert_eq!(Some(initial_retry - 1), new_retry);
		}
	});
}

// Replicating the actual flow with context to sucessfully execute complete_transfer
#[test]
fn complete_transfer_valid_flow() {
	minimal_test_ext().execute_with(|| {
		let claimer_icon = samples::ICON_ADDRESS[0];
		let claimer_ice = samples::ACCOUNT_ID[0];
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();
		let server_response = samples::SERVER_DATA[0];

		// Set some balance to creditor account first
		credit_creditor(10_00_000_u32);

		// Simulate we have done claim_request by actually adding it to
		// both snapshot map and pending queue
		pallet_airdrop::PendingClaims::<Test>::insert(&bl_num, &claimer_icon, 1_u8);
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&claimer_icon,
			types::SnapshotInfo::<Test>::default().ice_address(claimer_ice.clone()),
		);

		// Record free balance of both party before transaction
		let (pre_system_balance, pre_user_balance) = (
			get_free_balance(&AirdropModule::get_creditor_account()),
			get_free_balance(&claimer_ice),
		);

		// Do actual transfer and make sure execution passes
		let transfer_res = AirdropModule::complete_transfer(
			Origin::root(),
			bl_num,
			claimer_icon.clone(),
			server_response.clone(),
		);
		assert_ok!(transfer_res);

		// Record free balance of both party after transaction
		let (post_system_balance, post_user_balance) = (
			get_free_balance(&AirdropModule::get_creditor_account()),
			get_free_balance(&claimer_ice),
		);

		// Bhavioural expectation after complete_transfer is called
		{
			// Make sure user got right amount of money
			assert_eq!(post_user_balance, server_response.amount.into());

			// System balance is only redueced by balance transferred only as fee is 0 for this call
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
				AirdropModule::get_pending_claims(&bl_num, &claimer_icon),
				None
			);
			// Make sure this function update the snapshot mapping
			assert!(
				AirdropModule::get_icon_snapshot_map(&claimer_icon)
					.unwrap()
					.claim_status
			);
		}
	});
}

#[test]
fn donate_to_creditor() {
	minimal_test_ext().execute_with(|| {
		let donator = samples::ACCOUNT_ID[1];
		let creditor_account = AirdropModule::get_creditor_account();
		let donating_amount: types::BalanceOf<Test> = 1_00_000_u32.into();

		// Make sure donation will succeed as expected
		{
			// First create a user with some prefunded balance
			assert_ok!(pallet_balances::Pallet::<Test>::set_balance(
				Origin::root(),
				donator.clone(),
				donating_amount * 2_128,
				10_000_u32.into(),
			));

			// Record creditor balance before receiving donation
			let pre_donation_balance = get_free_balance(&creditor_account);

			// Do the donation and esnure it is ok
			assert_ok!(AirdropModule::donate_to_creditor(
				Origin::signed(donator.clone()),
				donating_amount,
				false
			));

			// Record creditor balance after receiving donaiton
			let post_donation_balance = get_free_balance(&creditor_account);

			// Make sure creditor is credited to exact donating amount
			assert_eq!(
				donating_amount,
				post_donation_balance - pre_donation_balance
			);
		}
	});
}

fn credit_creditor(balance: u32) {
	let creditor_account = AirdropModule::get_creditor_account();
	let deposit_res = <Test as pallet_airdrop::Config>::Currency::set_balance(
		mock::Origin::root(),
		creditor_account,
		balance.into(),
		10_000_u32.into(),
	);

	assert_ok!(deposit_res);
	assert_eq!(get_free_balance(&creditor_account), balance.into());
}

fn get_free_balance(account: &types::AccountIdOf<Test>) -> types::BalanceOf<Test> {
	<Test as pallet_airdrop::Config>::Currency::free_balance(account)
}
