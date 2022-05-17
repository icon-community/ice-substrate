// This file is part of ICE.

// Copyright (C) 2021-2022 ICE Network.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use frame_support::dispatch::EncodeLike;

use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, VestingSchedule},
};

use pallet_vesting::{Vesting as VestingStorage, *};

use crate::mock::{Balances, ExtBuilder, System, Test, Vesting};
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

	assert!(!<VestingStorage<T>>::contains_key(account));
}

#[test]
fn usable_balance_for_fees_during_vesting() {
	ExtBuilder::default().existential_deposit(ED).build().execute_with(|| {
		vest_and_assert_no_vesting::<Test>(5);

		// Make the schedule for the new transfer.
		let vesting_schedule = VestingInfo::new(
			ED * 10, // 256 * 10
			20,
			5,
		);

		// Account 5 should not have any vesting yet.
		assert_eq!(Vesting::vesting(&5), None);
		assert_eq!(Balances::usable_balance_for_fees(&5), 0);

		//transfer vesting balance
		assert_ok!(Vesting::vested_transfer(Some(3).into(), 5, vesting_schedule));
		assert_eq!(Vesting::vesting_balance(&5), Some(2560));

		System::set_block_number(2);

		assert_eq!(Balances::usable_balance(&5), 0);
		assert_eq!(Balances::free_balance(&5), 2560);

		// Account 5 cannot send more than vested amount, nothing has been vested yet
		assert_noop!(
			Balances::transfer(Some(5).into(), 3, 10),
			pallet_balances::Error::<Test, _>::LiquidityRestrictions,
		);

		System::set_block_number(6); // first vesting schedule starts after block 5
		assert_ok!(Vesting::vest(Some(5).into())); // vest 20 units
		assert_ok!(Balances::transfer(Some(5).into(), 3, 20));
	});
}
