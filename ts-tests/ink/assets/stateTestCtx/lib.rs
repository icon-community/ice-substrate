#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod state_test {
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::vec;
    use ink_prelude::string::ToString;
    use ink_storage::traits::{SpreadLayout};


    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo,ink_storage::traits::StorageLayout)
    )]
    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
    )]
    enum Status {
        Valid,
        Invalid,
    }

    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo,ink_storage::traits::StorageLayout,)
    )]
    pub struct S1 {
        id: u32,
        status: Status,
        str_arr: [String;2],
    }

    #[ink(storage)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo)
    )]
    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
    )]
    pub struct StateTest {
        msg: String,
        u8_arr: [u8;3],
        value: i128,
        is_true: bool,
        my_account: AccountId,
        my_balance: Balance,
        my_hash: Hash,
        my_vec: Vec<i16>,
        my_struct: S1,
    }

    impl StateTest {
        #[ink(constructor)]
        pub fn new(acc: AccountId, hash_val: Hash) -> Self {
            StateTest {
                msg: "ICE/SNOW Network".to_string(),
                value: 170141183460469231731687303715884105727,
                u8_arr:  [1,2,3],
                is_true: true,
                my_account: acc,
                my_balance: 123123123123,
                my_hash: hash_val,
                my_vec: vec![1,2,3,4,5,6,7,8,9,10,11],
                my_struct: S1 {
                    id: 1,
                    status: Status::Invalid,
                    str_arr: ["Str1".to_string(), "Str2".to_string()],
                }
            }
        }
        
        #[ink(message)]
        pub fn get(&self) -> StateTest {
            self.clone()
        }
    }
}