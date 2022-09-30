#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod staking_rewards {
	use ink_prelude::{vec, vec::Vec};
	use ink_storage::{
		traits::{PackedLayout, SpreadAllocate, SpreadLayout},
		Mapping,
	};

	const MIL: u128 = 1_000_000;
	const MAX_PERCENT: u128 = 100_000;

	#[ink(event)]
	pub struct DepositSuccessful {
		staker: AccountId,
		lock_box: LockBox,
	}

	#[ink(event)]
	pub struct RedeemSuccessful {
		staker: AccountId,
		lock_box: LockBox,
	}

	#[ink(event)]
	pub struct WithdrawSuccessful {
		staker: AccountId,
		lock_box: LockBox,
	}

	#[derive(
		Clone, Copy, Debug, PartialEq, scale::Decode, scale::Encode, PackedLayout, SpreadLayout,
	)]
	#[cfg_attr(
		feature = "std",
		derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
	)]
	pub struct LockBox {
		created_at: Timestamp,
		deposit: Balance,
		interest: Balance,
		release: Timestamp,
	}

	#[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
	pub enum Error {
		DepositDeadlinePassed,
		DepositWithoutValue,
		DepositTooBigWouldOverflow,
		LockBoxNotFound,
		LockBoxNotReleased,
	}

	#[ink(storage)]
	#[derive(Default, SpreadAllocate)]
	pub struct StakingRewards {
		owner: AccountId,
		max_deposit_value: u128,
		locking_duration: u64,
		deposit_deadline: u64,
		base_interest: u128,
		stakers_rate_permil: u128,
		stakers_sample: u128,
		liquidity_rate_permil: u128,
		liquidity_sample: u128,
		total_liquidity: u128,
		stakers_count: u128,
		lock_boxes: Mapping<AccountId, Vec<LockBox>>,
	}

	impl StakingRewards {
		#[ink(constructor, payable)]
		pub fn new(
			max_deposit_value: u128,
			locking_duration: u64,
			deposit_deadline: u64,
			base_interest: u128,
			stakers_rate_permil: u128,
			stakers_sample: u128,
			liquidity_rate_permil: u128,
			liquidity_sample: u128,
		) -> Self {
			ink_lang::utils::initialize_contract(|contract: &mut Self| {
				contract.owner = Self::env().caller();
				contract.max_deposit_value = max_deposit_value;
				contract.locking_duration = locking_duration;
				contract.deposit_deadline = deposit_deadline;
				contract.base_interest = base_interest;
				contract.stakers_rate_permil = stakers_rate_permil;
				contract.stakers_sample = stakers_sample;
				contract.liquidity_rate_permil = liquidity_rate_permil;
				contract.liquidity_sample = liquidity_sample;
				contract.total_liquidity = 0;
				contract.stakers_count = 0;
			})
		}

		#[ink(message, payable)]
		pub fn deposit(&mut self) -> Result<LockBox, Error> {
			let caller = self.env().caller();
			self.ensure_not_self_account(&caller);

			let now = self.env().block_timestamp();
			if now > self.deposit_deadline {
				return Err(Error::DepositDeadlinePassed);
			}

			let value = self.env().transferred_value();
			if value == 0 {
				return Err(Error::DepositWithoutValue);
			}

			if value > self.max_deposit_value {
				return Err(Error::DepositTooBigWouldOverflow);
			}

			let lock_box = LockBox {
				created_at: now,
				deposit: value,
				interest: value * self.interest_percent() / MAX_PERCENT,
				release: now + self.locking_duration,
			};

			self.add_box(&caller, lock_box.clone());

			self.total_liquidity += value;

			self.env().emit_event(DepositSuccessful {
				staker: caller,
				lock_box: lock_box.clone(),
			});

			Ok(lock_box)
		}

		#[ink(message)]
		pub fn redeem(&mut self, box_index: u128) -> Result<Balance, Error> {
			let caller = self.env().caller();
			self.ensure_not_self_account(&caller);

			let lock_box = self.remove_box(&caller, box_index as usize, true);
			if lock_box.is_err() {
				return Err(Error::LockBoxNotFound);
			}

			let lock_box = lock_box.unwrap();

			let amount = lock_box.deposit + lock_box.interest;

			self.total_liquidity -= &lock_box.deposit;

			self.transfer(caller.clone(), amount);

			self.env().emit_event(RedeemSuccessful {
				staker: caller,
				lock_box,
			});

			Ok(amount)
		}

		#[ink(message)]
		pub fn early_withdraw(&mut self, box_index: u128) -> Result<Balance, Error> {
			let caller = self.env().caller();
			self.ensure_not_self_account(&caller);

			let lock_box = self.remove_box(&caller, box_index as usize, false);
			if lock_box.is_err() {
				return Err(Error::LockBoxNotFound);
			}

			let lock_box = lock_box.unwrap();

			let amount = lock_box.deposit;

			self.total_liquidity -= &lock_box.deposit;

			self.transfer(caller.clone(), amount);

			self.env().emit_event(WithdrawSuccessful {
				staker: caller,
				lock_box,
			});

			Ok(amount)
		}

		#[ink(message)]
		pub fn refund(&mut self, amount: u128) -> Result<(), Error> {
			let caller = Self::env().caller();
			self.ensure_owner(&caller);

			assert!(amount <= self.env().balance(), "insufficient funds!");

			self.transfer(caller, amount);

			Ok(())
		}

		#[ink(message)]
		pub fn get_boxes(&self, account_id: AccountId) -> Option<Vec<LockBox>> {
			self.lock_boxes.get(&account_id)
		}

		fn interest_percent(&mut self) -> u128 {
			let negative_interest =
				self.stakers_count / self.stakers_sample * self.stakers_rate_permil / MIL
					+ self.total_liquidity / self.liquidity_sample * self.liquidity_rate_permil
						/ MIL;

			if self.base_interest >= negative_interest {
				self.base_interest - negative_interest
			} else {
				0
			}
		}

		fn add_box(&mut self, account: &AccountId, lock_box: LockBox) {
			let boxes = self.lock_boxes.get(&account);

			match boxes {
				Some(mut boxes) => {
					boxes.push(lock_box);
					self.lock_boxes.insert(&account, &boxes);
				}
				None => {
					self.stakers_count += 1;
					self.lock_boxes.insert(&account, &vec![lock_box]);
				}
			}
		}

		fn remove_box(
			&mut self,
			account: &AccountId,
			box_index: usize,
			should_check_release: bool,
		) -> Result<LockBox, Error> {
			let boxes = self.lock_boxes.get(&account);

			if boxes.is_none() {
				return Err(Error::LockBoxNotFound);
			}

			let mut boxes = boxes.unwrap();

			if box_index >= boxes.len() {
				return Err(Error::LockBoxNotFound);
			}

			let lock_box = boxes[box_index];
			if should_check_release {
				let now = self.env().block_timestamp();

				if now < lock_box.release {
					return Err(Error::LockBoxNotReleased);
				}
			}

			if boxes.len() != 1 {
				boxes.swap_remove(box_index);
				self.lock_boxes.insert(&account, &boxes);
			} else {
				self.stakers_count -= 1;
				self.lock_boxes.remove(&account);
			};

			Ok(lock_box)
		}

		fn transfer(&mut self, account: AccountId, amount: u128) {
			if self.env().transfer(account, amount).is_err() {
				panic!("Could not perform transfer");
			}
		}

		fn ensure_not_self_account(&mut self, account: &AccountId) {
			assert_ne!(
				account,
				&self.env().account_id(),
				"account should not be the own smart contract"
			);
		}

		fn ensure_owner(&mut self, account: &AccountId) {
			assert_eq!(account, &self.owner, "account is not owner");
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		use ink_env::{test, AccountId, DefaultEnvironment};

		const MAX_DEPOSIT_VALUE: u128 = u128::MAX / 5_000;
		const INITIAL_BALANCE: u128 = 5;

		#[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
		#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
		pub enum TestError {
			AccountNotFound,
		}

		fn default_accounts() -> test::DefaultAccounts<DefaultEnvironment> {
			test::default_accounts::<DefaultEnvironment>()
		}

		fn set_caller(acc_id: AccountId) {
			test::set_caller::<DefaultEnvironment>(acc_id);
		}

		fn set_callee(acc_id: AccountId) {
			test::set_callee::<DefaultEnvironment>(acc_id);
		}

		fn set_value_transferred(value: u128) {
			test::set_value_transferred::<DefaultEnvironment>(value);
		}

		fn set_account_balance(account_id: AccountId, value: u128) {
			test::set_account_balance::<DefaultEnvironment>(account_id, value);
		}

		fn get_account_balance(account_id: AccountId) -> Result<u128, TestError> {
			test::get_account_balance::<DefaultEnvironment>(account_id)
				.map_err(|_| TestError::AccountNotFound)
		}

		fn alice_id() -> AccountId {
			default_accounts().alice
		}

		fn bob_id() -> AccountId {
			default_accounts().bob
		}

		fn charlie_id() -> AccountId {
			default_accounts().charlie
		}

		fn contract_id() -> AccountId {
			charlie_id()
		}

		fn owner_id() -> AccountId {
			alice_id()
		}

		fn advance_block() {
			test::advance_block::<DefaultEnvironment>();
		}

		fn build_contract() -> StakingRewards {
			set_caller(owner_id());
			set_account_balance(contract_id(), INITIAL_BALANCE);
			StakingRewards::new(MAX_DEPOSIT_VALUE, 6, 12, 5_000, 0, 10, 0, 10)
		}

		#[test]
		fn test_deposit_wait_redeem() {
			let mut sc = build_contract();

			set_caller(bob_id());
			set_callee(contract_id());
			set_value_transferred(100);

			assert!(sc.deposit().is_ok());

			advance_block();
			advance_block();

			set_account_balance(contract_id(), 1000u128);
			assert_eq!(get_account_balance(contract_id()).unwrap(), 1000u128);

			set_caller(bob_id());
			set_callee(contract_id());
			assert!(sc.redeem(0).is_ok());
			assert_eq!(sc.redeem(0), Err(Error::LockBoxNotFound));

			assert_eq!(get_account_balance(bob_id()).unwrap(), 105u128);

			set_caller(owner_id());
			set_callee(contract_id());
			assert!(sc.refund(895u128).is_ok());

			assert_eq!(get_account_balance(owner_id()).unwrap(), 895u128);
			assert_eq!(get_account_balance(contract_id()).unwrap(), 0u128);
		}

		#[test]
		fn test_lock_max_amount_should_work() {
			let mut sc = build_contract();

			set_caller(bob_id());
			set_callee(contract_id());
			set_value_transferred(MAX_DEPOSIT_VALUE);
			assert!(sc.deposit().is_ok());
		}

		#[test]
		fn test_deposit_above_max_should_not_work() {
			let mut sc = build_contract();

			set_caller(bob_id());
			set_callee(contract_id());
			set_value_transferred(MAX_DEPOSIT_VALUE + 1);
			assert_eq!(sc.deposit(), Err(Error::DepositTooBigWouldOverflow));
		}
	}
}
