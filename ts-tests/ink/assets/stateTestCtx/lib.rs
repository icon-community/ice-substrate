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
    pub struct IntStruct {
        my_u8: u8,
        my_u16: u16,
        my_u32: u32,
        my_u64: u64,
        my_u128: u128,
        my_i8: i8,
        my_i16: i16,
        my_i32: i32,
        my_i64: i64,
        my_i128: i128,
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
        my_int_struct: IntStruct,
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
                },
                my_int_struct: IntStruct {
                    my_u8: 255,
                    my_u16: 65535,
                    my_u32: 4_294_967_295,
                    my_u64: 18_446_744_073_709_551_615,
                    my_u128: 340_282_366_920_938_463_463_374_607_431_768_211_455,
                    my_i8: 127,
                    my_i16: 32767,
                    my_i32: 2147483647,
                    my_i64: 9_223_372_036_854_775_807,
                    my_i128: 170_141_183_460_469_231_731_687_303_715_884_105_727,
                }
            }
        }

        #[ink(message)]
        pub fn get(&self) -> StateTest {
            self.clone()
        }
    }
}