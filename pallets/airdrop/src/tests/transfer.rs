use super::prelude::*;
use frame_support::traits::tokens::currency::Currency;
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
					samples::ACCOUNT_ID[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// Make sure sudo cal call this
		{
			assert_storage_noop!(assert_ne! {
				AirdropModule::complete_transfer(
					Origin::signed(AirdropModule::get_sudo_account()),
					1_u32.into(),
					samples::ACCOUNT_ID[0],
					samples::SERVER_DATA[0]
				)
				.unwrap_err(),

				PalletError::DeniedOperation.into()
			});
		}

		// Make sure other signed user can't called this
		{
			let non_sudo = not_airdrop_sudo(samples::ACCOUNT_ID[2]);
			assert_storage_noop!(assert_eq! {
				AirdropModule::complete_transfer(
					Origin::signed(non_sudo),
					1_u32.into(),
					samples::ACCOUNT_ID[0],
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
					samples::ACCOUNT_ID[0],
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
		let claimer = samples::ACCOUNT_ID[1];
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
		let claimer = samples::ACCOUNT_ID[1];

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
fn insufficient_creditor_balance() {
	minimal_test_ext().execute_with(|| {
		let claimer = samples::ACCOUNT_ID[1];
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
				claimer,
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
		let claimer = samples::ACCOUNT_ID[1].clone();
		let bl_num: types::BlockNumberOf<Test> = 1_u32.into();
		let server_response = samples::SERVER_DATA[0];

		// Set some balance to creditor account first
		credit_creditor(10_00_000_u32);

		// Simulate we have done claim_request by actually adding it to
		// both snapshot map and pending queue
		pallet_airdrop::PendingClaims::<Test>::insert(&bl_num, &claimer, 1_u8);
		pallet_airdrop::IceSnapshotMap::<Test>::insert(
			&claimer,
			types::SnapshotInfo::<Test>::default(),
		);

		// Record free balance of both party before transaction
		let (pre_system_balance, pre_user_balance) = (
			get_free_balance(&AirdropModule::get_creditor_account()),
			get_free_balance(&claimer),
		);

		// Do actual transfer and make sure execution passes
		let transfer_res = AirdropModule::complete_transfer(
			Origin::root(),
			bl_num,
			claimer.clone(),
			server_response.clone(),
		);
		assert_ok!(transfer_res);

		// Record free balance of both party after transaction
		let (post_system_balance, post_user_balance) = (
			get_free_balance(&AirdropModule::get_creditor_account()),
			get_free_balance(&claimer),
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
			assert_eq!(AirdropModule::get_pending_claims(&bl_num, &claimer), None);
			// Make sure this function update the snapshot mapping
			assert!(
				AirdropModule::get_ice_snapshot_map(&claimer)
					.unwrap()
					.claim_status
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
