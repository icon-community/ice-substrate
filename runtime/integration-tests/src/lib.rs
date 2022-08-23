#![cfg(test)]

use arctic_runtime::{AccountId, Balance, CurrencyId, TokenInfo, TreasuryPalletId};
use sp_runtime::traits::AccountIdConversion;

mod rococo_cross_chain_xfer;
mod rococo_testnet;

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];

pub fn dollar(currency_id: CurrencyId) -> Balance {
	10u128.saturating_pow(currency_id.decimals().unwrap_or(12).into())
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![TreasuryPalletId::get().into_account_truncating()]
}
