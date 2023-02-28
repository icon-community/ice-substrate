#![cfg(test)]

use arctic_runtime::{AccountId, Balance, CurrencyId, TokenInfo, TreasuryPalletId};
use sp_runtime::traits::AccountIdConversion;
use polkadot_parachain::primitives::Id as ParaId;

mod setup;
mod relay;
mod trappist_relay;
mod para;
mod rococo_cross_chain_xfer;
mod rococo_testnet;
mod kusama_testnet;
mod kusama_cross_chain_xfer;
mod testrelaywithtestpara_cross_chain_xfer;
mod testrelaywithtestpara_testnet;
mod testrelaywitharctic_testnet;
mod testrelaywitharctic_cross_chain_xfer;
mod trappist_testnet;
mod trappist_cross_chain_transfer;
use xcm::latest::prelude::*;
<<<<<<< Updated upstream
=======
use std::sync::Once;

>>>>>>> Stashed changes
// pub const ALICE: [u8; 32] = [0u8; 32];
pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const ALICE_RELAY: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([1u8; 32]);
pub const BOB: [u8; 32] = [1u8; 32];

pub const INITIAL_BALANCE: u128 = 4_000_000_000_000_000_000;
// not sure where this comes from; but asserting the para balancing on this value succeeds
pub const PARA_BALANCE: u128 = 4294_967_296_000_000_000_000_000_000;
pub const ASSET_RESERVE_PARA_ID: u32 = 2001;

// pub type RelayChainPalletXcm = pallet_xcm::Pallet<rococo_runtime::Runtime>;
pub type RelayChainPalletXcm = pallet_xcm::Pallet<relay::Runtime>;
pub type RococoPalletXcm = pallet_xcm::Pallet<rococo_runtime::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<arctic_runtime::Runtime>;

pub fn dollar(currency_id: CurrencyId) -> Balance {
	10u128.saturating_pow(currency_id.decimals().unwrap_or(12).into())
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![TreasuryPalletId::get().into_account_truncating()]
}

pub fn para_account_id(id: u32) -> relay::AccountId {
	ParaId::from(id).into_account_truncating()
}
fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
    BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

static INIT: Once = Once::new();
fn init_tracing() {
	INIT.call_once(|| {
		// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
		let _ = tracing_subscriber::fmt()
			.with_max_level(tracing::Level::TRACE)
			.with_env_filter("xcm=trace,system::events=trace") // Comment out this line to see all traces
			.with_test_writer()
			.init();
	});
}
