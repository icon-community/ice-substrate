#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod keccak_test {

    use ink_env::hash;
    use ink_prelude::string::ToString;

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
    pub struct KeccakTest {
        hash: [u8;32],
        value: i32
    }

    #[ink(event)]
	pub struct Operating {}

    impl KeccakTest {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            KeccakTest { 
                value: init_value , 
                hash:  <hash::Keccak256 as hash::HashOutput>::Type::default(),
            }
        }

        fn hash_keccak_256(input: &[u8]) -> [u8; 32] {
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink_env::hash_bytes::<hash::Keccak256>(input, &mut output);
            output
        }

        #[ink(message)]
        pub fn operate(&mut self) {
            self.env().emit_event(Operating{});
            self.value = self.value * 4 - 2;
            let val = self.value.to_string();
            let bytes= val.as_bytes();
            self.hash = Self::hash_keccak_256(bytes);
        }

        #[ink(message)]
        pub fn get(&self) -> KeccakTest {
            self.clone()
        }
    }
}