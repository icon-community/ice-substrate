#![cfg_attr(not(feature = "std"), no_std)]

pub use self::adder::{
    Adder,
    AdderRef,
};

use ink_lang as ink;

#[ink::contract]
mod adder {
    use accumulator::AccumulatorRef;

    /// Increments the underlying `accumulator` value.
    #[ink(storage)]
    pub struct Adder {
        /// The `accumulator` to store the value.
        accumulator: AccumulatorRef,
    }

    #[ink(event)]
	pub struct DepositSuccessful {
        depositer: AccountId,
		amount: u128,
	}

    impl Adder {
        /// Creates a new `adder` from the given `accumulator`.
        #[ink(constructor)]
        pub fn new(init_value: i32, version: u32, accumulator_code_hash: Hash) -> Self {
            let salt = version.to_le_bytes();
            let accumulator = AccumulatorRef::new(init_value)
                .endowment(0)
                .code_hash(accumulator_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!("failed at instantiating the accumulator contract: {:?}", error)
                });
            Self { accumulator }
        }

        /// Increases the `accumulator` value by some amount.
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.accumulator.inc(by)
        }

        #[ink(message)]
        pub fn expensive_func(&mut self) {
            let mut i:u64 = 0;

            while i <= 1_000_000_000_000 {
                self.accumulator.inc(1);
                i = i + 1;
            }

        }

        #[ink(message, payable)]
        pub fn receive_funds(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_value();

            self.env().emit_event(DepositSuccessful {
                depositer: caller, 
                amount: value,
            });
        }

        #[ink(message)]
        pub fn tear_down(&mut self) {
            let caller = self.env().caller();
            self.env().terminate_contract(caller);
        }
    }
}
