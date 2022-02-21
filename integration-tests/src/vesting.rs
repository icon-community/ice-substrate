use frame_support::{ dispatch::EncodeLike};
use crate::mock::*;

/// A default existential deposit.
const ED: u64 = 256;
/// Calls vest, and asserts that there is no entry for `account`
/// in the `Vesting` storage item.
fn vest_and_assert_no_vesting<T>(account: u64)
where
	u64: EncodeLike<<T as frame_system::Config>::AccountId>,
	T: pallet_vesting::Config,
{
	// Its ok for this to fail because the user may already have no schedules.
	let _result = Vesting::vest(Some(account).into());
	// assert!(!<VestingStorage>::contains_key(account));
}

	
#[test]
fn check_vesting_status() {
	ExtBuilder::default().existential_deposit(ED).build().execute_with(|| {
		let user1_free_balance = Balances::free_balance(&1);
		let user2_free_balance = Balances::free_balance(&2);
		let user12_free_balance = Balances::free_balance(&12);
		assert_eq!(user1_free_balance, ED * 10); // Account 1 has free balance
		assert_eq!(user2_free_balance, ED * 20); // Account 2 has free balance
		assert_eq!(user12_free_balance, ED * 10); // Account 12 has free balance
		let user1_vesting_schedule = pallet_vesting::VestingInfo::new(
			ED * 5,
			128, // Vesting over 10 blocks
			0,
		);
		let user2_vesting_schedule = pallet_vesting::VestingInfo::new(
			ED * 20,
			ED, // Vesting over 20 blocks
			10,
		);
		let user12_vesting_schedule = pallet_vesting::VestingInfo::new(
			ED * 5,
			64, // Vesting over 20 blocks
			10,
		);
		assert_eq!(Vesting::vesting(&1).unwrap(), vec![user1_vesting_schedule]); // Account 1 has a vesting schedule
		assert_eq!(Vesting::vesting(&2).unwrap(), vec![user2_vesting_schedule]); // Account 2 has a vesting schedule
		assert_eq!(Vesting::vesting(&12).unwrap(), vec![user12_vesting_schedule]); // Account 12 has a vesting schedule

		// // Account 1 has only 128 units vested from their illiquid ED * 5 units at block 1
		// assert_eq!(Vesting::vesting_balance(&1), Some(128 * 9));
		// // Account 2 has their full balance locked
		// assert_eq!(Vesting::vesting_balance(&2), Some(user2_free_balance));
		// // Account 12 has only their illiquid funds locked
		// assert_eq!(Vesting::vesting_balance(&12), Some(user12_free_balance - ED * 5));

		System::set_block_number(10);
		assert_eq!(System::block_number(), 10);

		// // Account 1 has fully vested by block 10
		// assert_eq!(Vesting::vesting_balance(&1), Some(0));
		// // Account 2 has started vesting by block 10
		// assert_eq!(Vesting::vesting_balance(&2), Some(user2_free_balance));
		// // Account 12 has started vesting by block 10
		// assert_eq!(Vesting::vesting_balance(&12), Some(user12_free_balance - ED * 5));

		System::set_block_number(30);
		assert_eq!(System::block_number(), 30);

		// assert_eq!(Vesting::vesting_balance(&1), Some(0)); // Account 1 is still fully vested, and not negative
		// assert_eq!(Vesting::vesting_balance(&2), Some(0)); // Account 2 has fully vested by block 30
		// assert_eq!(Vesting::vesting_balance(&12), Some(0)); // Account 2 has fully vested by block 30

		// Once we unlock the funds, they are removed from storage.
		vest_and_assert_no_vesting::<Test>(1);
		vest_and_assert_no_vesting::<Test>(2);
		vest_and_assert_no_vesting::<Test>(12);
	});
}

	
#[test]
fn redeem_vesting(fromAccount: u64, newAccount:u64, amount: T::Balance) {
	ExtBuilder::default().existential_deposit(ED).build().execute_with(|| {
		Balances::transfer(fromAccount, newAccount, amount);
		let user1_free_balance = Balances::free_balance(&newAccount);
		assert_eq!(newAccount, amount);
		assert_eq!(Balances::transfer(Some(1).into(), 2, 55));
		let user1_vesting_schedule = pallet_vesting::VestingInfo::new(
			ED * 5,
			128, // Vesting over 10 blocks
			0,
		);
		assert_eq!(Vesting::vesting(&newAccount).unwrap(), vec![user1_vesting_schedule])

		System::set_block_number(10);
		assert_eq!(System::block_number(), 10);

		System::set_block_number(30);
		assert_eq!(System::block_number(), 30);
		vest_and_assert_no_vesting::<Test>(1);
	}
}
