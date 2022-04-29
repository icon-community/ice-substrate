use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};

use crate::{Balance};

pub mod currency_base {
    use super::*;
    pub const DOLLARS: Balance = 1_000_000_000_000_000_000; // 10u128.pow(18)
    pub const CENTS: Balance = DOLLARS / 100; // 10_000_000_000_000_000
    pub const MILLICENTS: Balance = CENTS / 1000; // 10_000_000_000_000
    pub const MICROCENTS: Balance = MILLICENTS / 1000; // 10_000_000_000
}

pub const ICZ: Balance = currency_base::DOLLARS;

#[cfg(test)]
mod tests {
	use super::*;
	// TODO: static assert
	#[allow(clippy::assertions_on_constants)]
	#[test]
	fn blocks_per_year_saturation() {
		assert!(ICZ < u128::MAX);
		assert_eq!(10u128.pow(18), ICZ); // 1 ICY 1_000_000_000_000_000_000
		//assert_eq!(1_000_000_000_000, MILLIICZ); // 0.000_0001 ICZ
        print!("ICZ {}", ICZ);
	}
}