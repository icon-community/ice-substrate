pub(crate) const EXCHANGE_ACCOUNTS: &[([u8; 20], u128)] = &[
	(
		hex_literal::hex!("562dc1e2c7897432c298115bc7fbcc3b9d5df294"),
		70717613544517522852341727,
	),
	(
		hex_literal::hex!("61acc986a761b5f354dc8777360aeaf47b2ab616"),
		8968750000000000000,
	),
	(
		hex_literal::hex!("6d14b2b77a9e73c5d5804d43c7e3c3416648ae3d"),
		8348890436199324029817984,
	),
	(
		hex_literal::hex!("938b9a413de9ffbbeae72e7034931a3bdf0f1e96"),
		2959971579000000000000000 + 13972742011003245391080,
	),
	(
		hex_literal::hex!("d182113fea7ae3164871bfda90ec8652123aa354"),
		352948797792142357220773,
	),
];

use crate::{types, Config};
use sp_runtime::traits::Convert;
use sp_std::vec::Vec;
pub(crate) fn get_exchange_account<T: Config>() -> Vec<(types::IconAddress, types::BalanceOf<T>)> {
	EXCHANGE_ACCOUNTS
		.iter()
		.map(|(address, balance)| {
			let address = *address;
			let balance: types::BalanceOf<T> = T::BalanceTypeConversion::convert(*balance);
			(address, balance)
		})
		.collect()
}
