#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod migration_test {

    use ink_env::hash;
    use ink_prelude::string::String;
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

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
        Clone
    )]
    pub struct MigrationTest {
        msg: String,
        hash: [u8;32],
        value: u8,
        structure:TestStruct,
    }

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
        PackedLayout,
        SpreadAllocate,
    )]
    pub struct TestStruct{
        pub val:u8,
        pub name:String,
    }

    #[ink(event)]
	pub struct Operating {}

    impl MigrationTest {
        #[ink(constructor)]
        pub fn new(init_msg: String, init_value: u8) -> Self {
            MigrationTest { 
                msg: init_msg.clone(), 
                value: init_value , 
                hash:  [init_value;32],
                structure: TestStruct{
                    val:init_value,
                    name:init_msg,
                }
            }
        }

       #[ink(message)]
        pub fn get(&self) -> MigrationTest {
            self.clone()
        }

        #[ink(message)]
        pub fn set_state(&mut self, msg:String,val:u8) {
            self.msg=msg.clone();
            self.value=val;
            self.hash=[val;32];
            self.structure=TestStruct{
                val:val,
                name:msg
            }
        }
    }
}