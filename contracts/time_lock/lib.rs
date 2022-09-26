#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod sc_deposit {
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use ink_storage::Mapping;

    #[ink(event)]
    pub struct DepositSuccessful {
        lock_box: LockBox,
        box_index: u128,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct RedeemSuccessful {
        lock_box: LockBox,
        box_index: u128,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct WithdrawSuccessful {
        lock_box: LockBox,
        box_index: u128,
        timestamp: Timestamp,
    }

    #[derive(Clone, Copy, scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct LockBox {
        beneficiary: AccountId,
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
        LockBoxNotOwned,
        LockBoxNotReleased,
        WithdrawingNotAllowed,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct TimeLock {
        owner: AccountId,
        interest_percent: u128,
        max_deposit_value: u128,
        locking_duration: u64,
        deposit_deadline: u64,
        lock_box_counter: u128,
        lock_boxes: Mapping<u128, LockBox>,
    }

    impl TimeLock {
        #[ink(constructor, payable)]
        pub fn new(
            interest_percent: u128,
            max_deposit_value: u128,
            locking_duration: u64,
            deposit_deadline: u64,
        ) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = Self::env().caller();
                contract.interest_percent = interest_percent;
                contract.max_deposit_value = max_deposit_value;
                contract.locking_duration = locking_duration;
                contract.deposit_deadline = deposit_deadline;
            })
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<u128, Error> {
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

            let caller = self.env().caller();
            let box_index = self.lock_box_counter;
            let lock_box = LockBox {
                beneficiary: caller,
                deposit: value,
                interest: value * self.interest_percent / 100,
                release: now + self.locking_duration,
            };

            self.lock_boxes.insert(box_index, &lock_box);
            self.lock_box_counter += 1;

            self.env().emit_event(DepositSuccessful {
                lock_box,
                box_index,
                timestamp: now,
            });

            Ok(box_index)
        }

        #[ink(message)]
        pub fn redeem(&mut self, box_index: u128) -> Result<Balance, Error> {
            if box_index >= self.lock_box_counter {
                return Err(Error::LockBoxNotFound);
            }

            let lock_box = self.lock_boxes.get(box_index);
            if lock_box.is_none() {
                return Err(Error::LockBoxNotFound);
            }

            let lock_box = lock_box.unwrap();
            let caller = self.env().caller();
            if lock_box.beneficiary != caller {
                return Err(Error::LockBoxNotOwned);
            }

            let now = self.env().block_timestamp();
            if now < lock_box.release {
                return Err(Error::LockBoxNotReleased);
            }

            self.lock_boxes.remove(box_index);

            let amount = lock_box.deposit + lock_box.interest;
            if self
                .env()
                .transfer(lock_box.beneficiary.clone(), amount)
                .is_err()
            {
                panic!("Could not perform transfer");
            }

            self.env().emit_event(RedeemSuccessful {
                lock_box,
                box_index,
                timestamp: now,
            });

            Ok(amount)
        }

        #[ink(message)]
        pub fn early_withdraw(&mut self, box_index: u128) -> Result<Balance, Error> {
            if box_index >= self.lock_box_counter {
                return Err(Error::LockBoxNotFound);
            }

            let lock_box = self.lock_boxes.get(box_index);
            if lock_box.is_none() {
                return Err(Error::LockBoxNotFound);
            }

            let lock_box = lock_box.unwrap();
            let caller = self.env().caller();
            if lock_box.beneficiary != caller {
                return Err(Error::LockBoxNotOwned);
            }

            self.lock_boxes.remove(box_index);

            let amount = lock_box.deposit;
            if self
                .env()
                .transfer(lock_box.beneficiary.clone(), amount)
                .is_err()
            {
                panic!("Could not perform transfer");
            }

            let now = self.env().block_timestamp();
            self.env().emit_event(WithdrawSuccessful {
                lock_box,
                box_index,
                timestamp: now,
            });

            Ok(lock_box.deposit)
        }

        #[ink(message)]
        pub fn refund(&mut self, amount: u128) -> Result<(), Error> {
            let caller = Self::env().caller();
            if caller != self.owner {
                return Err(Error::WithdrawingNotAllowed);
            }

            assert!(amount <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(caller, amount).is_err() {
                panic!("Could not perform transfer");
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_env::{test, AccountId, DefaultEnvironment};

        const MAX_DEPOSIT_VALUE: u128 = u128::MAX / 105;
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

        fn build_contract() -> TimeLock {
            set_caller(owner_id());
            set_account_balance(contract_id(), INITIAL_BALANCE);
            TimeLock::new(5, MAX_DEPOSIT_VALUE, 6, 12)
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
