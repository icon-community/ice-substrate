#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
mod contract_data;
mod constants;

#[ink::contract]
mod snow_rewards {
    use crate::{contract_data::{CONTRIBUTORS}, constants};
    use ::ink_env::{
        call::{build_call, Call, ExecutionInput, Selector},
        DefaultEnvironment,
    };
    use ink_env::CallFlags;
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct SnowRewards {
        owner: AccountId,
        operator: Option<AccountId>,
        whitelist: ink_storage::Mapping<AccountId, RewardInfo>,
        import_index: u32,
    }

    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
        PackedLayout,
        SpreadAllocate,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub struct RewardInfo {
        amount: u128,
        is_claimed: bool,
    }

    #[derive(
        scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub enum Error {
        TransferFailed,
        OperatorNotSet,
        NotAllowed,
        AlreadyClaimed,
        NotListed,
        InvalidTransferAmount,
        InsufficientBalance,
        ConversionFailed,
        DataNotInitialized,
    }
    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(
        scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub enum Outcome {
        Failed(Error),
        Success,
    }

    #[ink(event)]
    pub struct TransferFailed {
        error: Error,
    }

    #[ink(event)]
    pub struct ContractError {
        candidate: AccountId,
        error: Error,
    }

    #[ink(event)]
    pub struct TransferSuccess {
        account: AccountId,
        amount: u128,
    }

    impl SnowRewards {
        #[ink(constructor, payable)]
        pub fn new(version: u32) -> Self {
            let paid = Self::env().transferred_value();
            let salt = version.to_le_bytes();
            ink_env::debug_println!("Init Contract With Balance {:?}", Self::env().balance());
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = Self::env().caller();
                contract.operator = None;
                contract.whitelist = <ink_storage::Mapping<AccountId, RewardInfo>>::default();
                contract.import_index = 0;
            })
        }

        #[ink(message)]
        pub fn set_operator(&mut self, account: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAllowed);
            }
            if self.operator == None {
                self.init()?;
            }
            self.operator = Some(account);
            Ok(())
        }

        #[ink(message)]
        pub fn tear_down(&mut self) {
            let caller = self.env().caller();
            self.ensure_owner(&caller);
            self.env().terminate_contract(caller);
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAllowed);
            }
            ink_env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_operator(&self) -> Option<AccountId> {
            self.operator
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner.clone()
        }

        #[ink(message)]
        pub fn has_claimed(&self, candidate: AccountId) -> bool {
            self.whitelist
                .get(&candidate)
                .map_or(false, |r| r.is_claimed)
        }

        #[ink(message)]
        pub fn get_info(&self, candidate: AccountId) -> Option<RewardInfo> {
            self.whitelist.get(&candidate)
        }

        #[ink(message)]
        pub fn claim_enabled(&self) -> bool {
            return (CONTRIBUTORS.len() - 1)
                .try_into()
                .map_or(false, |len| self.import_index >= len);
        }

        #[ink(message)]
        pub fn distribute_reward(
            &mut self,
            candidate: AccountId,
            amount: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            if !self.claim_enabled() {
                return Err(Error::DataNotInitialized);
            }
            self.check_access(caller)?;

            let mut reward_info = self.check_whitelist(candidate, amount)?;

            if self.env().balance() < amount {
                return Err(Error::InsufficientBalance);
            }

            reward_info.is_claimed = true;
            self.whitelist.insert(&candidate, &reward_info);

            let transfer_result = self.make_payment(candidate, reward_info.amount);

            if transfer_result.is_err() {
                reward_info.is_claimed = false;
                self.whitelist.insert(&candidate, &reward_info);
                return Err(transfer_result.err().unwrap());
            }

            return Ok(());
        }

        pub fn balance(&self) -> Balance {
            return self.env().balance();
        }

        pub fn ensure_owner(&self, account: &AccountId) {
            assert_eq!(account, &self.owner, "account is not owner");
        }

        pub fn add_whitelist(&mut self, candidate: AccountId, amount: u128) {
            if !self.whitelist.contains(&candidate) {
                let reward_info = RewardInfo {
                    amount,
                    is_claimed: false,
                };
                self.whitelist.insert(&candidate, &reward_info)
            }
        }

        pub fn init(&mut self) -> Result<()> {
            let max_length: u32 = CONTRIBUTORS
                .len()
                .try_into()
                .map_err(|_e| Error::ConversionFailed)?;
            while self.import_index < (max_length - 1) {
                self.import_data(None)?;
            }
            Ok(())
        }

        pub fn check_whitelist(&self, candidate: AccountId, amount: u128) -> Result<RewardInfo> {
            match self.whitelist.get(&candidate) {
                Some(reward_info) => {
                    if reward_info.is_claimed == true {
                        return Err(Error::AlreadyClaimed);
                    }

                    if reward_info.amount != amount {
                        return Err(Error::InvalidTransferAmount);
                    }
                    Ok(reward_info)
                }
                None => Err(Error::NotListed),
            }
        }

        pub fn check_access(&self, caller: AccountId) -> Result<()> {
            match self.operator {
                Some(operator) => {
                    if caller != operator {
                        return Err(Error::NotAllowed);
                    }
                    Ok(())
                }
                None => Err(Error::OperatorNotSet),
            }
        }
        
        pub fn make_payment(&mut self, candidate: AccountId, amount: Balance) -> Result<()> {
            self.env().transfer(candidate, amount).map_err(|_e| Error::TransferFailed)
        }

        #[ink(message)]
        pub fn import_data(&mut self, range: Option<u32>) -> Result<()> {
            let start = self.import_index;
            let end_range = start + range.map_or(100, |v| v);
            let max_end = CONTRIBUTORS
                .len()
                .try_into()
                .map_err(|_e| Error::ConversionFailed)?;
            let end = if end_range > max_end {
                max_end
            } else {
                end_range
            };

            for i in start..end {
                let index: usize = i.try_into().map_err(|_e| Error::ConversionFailed)?;
                let contributor = CONTRIBUTORS[index];
                let candidate = AccountId::from(contributor.0);
                self.add_whitelist(candidate, contributor.1);
            }
            self.import_index = end;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn owner_is_logged() {
            let owner = AccountId::from([0x1; 32]);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(owner.clone());
            let snow_rewards = SnowRewards::new(1, None);
            println!("{:?}", &snow_rewards);
            assert_eq!(snow_rewards.get_owner(), owner);
        }

        #[ink::test]
        fn only_owner_can_set_operator() {
            let owner = AccountId::from([0x1; 32]);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(owner.clone());
            let mut snow_rewards = SnowRewards::new(1, None);
            let operator = AccountId::from([0x0; 32]);
            snow_rewards.set_operator(operator.clone()).unwrap();
            assert_eq!(snow_rewards.get_operator(), Some(operator));
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(AccountId::from([0x2; 32]));
            assert_eq!(
                snow_rewards.set_operator(operator.clone()),
                Err(Error::NotAllowed)
            );
        }

        #[ink::test]
        fn checks_insuffcient_balance() {
            let owner = AccountId::from([0x1; 32]);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(owner.clone());
            let mut snow_rewards = SnowRewards::new(1, None);
            let operator = AccountId::from([0x0; 32]);
            snow_rewards.set_operator(operator.clone()).unwrap();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(operator.clone());
            let candidate = AccountId::from([0x6; 32]);
            let contract_balance = snow_rewards.balance();
            let amount = contract_balance + 100;
            snow_rewards.add_whitelist(candidate, amount);
            assert_eq!(
                snow_rewards.distribute_reward(candidate, amount),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn checks_candidate_whitelisted() {
            let owner = AccountId::from([0x1; 32]);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(owner.clone());
            let mut snow_rewards = SnowRewards::new(1, None);
            let operator = AccountId::from([0x0; 32]);
            snow_rewards.set_operator(operator.clone()).unwrap();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(operator.clone());
            let candidate = AccountId::from([0x6; 32]);
            assert_eq!(
                snow_rewards.distribute_reward(candidate, 100),
                Err(Error::NotListed)
            );
            snow_rewards.add_whitelist(candidate, 90);
            assert_eq!(
                snow_rewards.distribute_reward(candidate, 100),
                Err(Error::InvalidTransferAmount)
            );
        }

        #[ink::test]
        fn test_distribute_reward() {
            let owner = AccountId::from([0x1; 32]);
            let candidate = AccountId::from([0x5; 32]);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(owner.clone());
            let mut snow_rewards = SnowRewards::new(1, None);

            assert_eq!(
                snow_rewards.distribute_reward(candidate, 100),
                Err(Error::DataNotInitialized)
            );

            let operator = AccountId::from([0x0; 32]);
            snow_rewards.set_operator(operator.clone()).unwrap();

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(AccountId::from([0x4; 32]));

            assert_eq!(
                snow_rewards.distribute_reward(candidate, 100),
                Err(Error::NotAllowed)
            );

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(operator.clone());
            ink_env::test::set_balance::<ink_env::DefaultEnvironment>(candidate, 0);

            snow_rewards.add_whitelist(candidate, 100);
            assert_eq!(snow_rewards.distribute_reward(candidate, 100), Ok(()));
            let balance_after =
                ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(candidate)
                    .unwrap();
            assert_eq!(balance_after, 100_u128);
            assert_eq!(snow_rewards.has_claimed(candidate), true);

            // prevents double claim

            assert_eq!(
                snow_rewards.distribute_reward(candidate, 100),
                Err(Error::AlreadyClaimed)
            );
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    }
}


