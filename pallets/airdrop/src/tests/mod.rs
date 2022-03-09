use crate::mock;
mod claim;
mod signature_validation;
mod transfer;
mod utility_functions;

pub mod prelude {
	pub use super::{
		assert_tx_call, get_last_event, minimal_test_ext, not_offchain_account, offchain_test_ext,
		put_response, run_to_block, samples,
	};
	pub use crate as pallet_airdrop;
	pub use crate::tests;
	pub use frame_support::{
		assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_noop, assert_ok,
		assert_storage_noop,
	};
	pub use hex_literal::hex as decode_hex;
	pub use pallet_airdrop::mock::{self, AirdropModule, Origin, Test};
	pub use pallet_airdrop::types;
	pub use sp_core::bytes;
	pub use sp_runtime::traits::IdentifyAccount;

	pub type PalletError = pallet_airdrop::Error<Test>;
	pub type PalletEvent = pallet_airdrop::Event<Test>;
	pub type PalletCall = pallet_airdrop::Call<Test>;
}
use mock::System;
use prelude::*;

pub mod samples {
	use super::decode_hex;
	use super::types::{IconAddress, ServerResponse};
	use sp_core::sr25519;

	pub const ACCOUNT_ID: &[sr25519::Public] = &[
		sr25519::Public([1; 32]),
		sr25519::Public([2; 32]),
		sr25519::Public([3; 32]),
		sr25519::Public([4; 32]),
		sr25519::Public([5; 32]),
	];

	pub const SERVER_DATA: &[ServerResponse] = &[
		ServerResponse {
			omm: 1234443_u128,
			amount: 345323_u128,
			stake: 8437566_u128,
			defi_user: true,
		},
		ServerResponse {
			omm: 8548467_u128,
			amount: 928333_u128,
			stake: 298329_u128,
			defi_user: false,
		},
	];

	pub const ICON_ADDRESS: &[IconAddress] = &[
		decode_hex!("ee1448f0867b90e6589289a4b9c06ac4516a75a9"),
		decode_hex!("ee33286f367b90e6589289a4b987a6c4516a753a"),
		decode_hex!("ee12463586abb90e6589289a4b9c06ac4516a7ba"),
		decode_hex!("ee02363546bcc50e643910104321c0623451a65a"),
	];
}

/// Dummy implementation for IconVerififable trait for test AccountId
/// This implementation always passes so should not be dependent upon
impl types::IconVerifiable for sp_core::sr25519::Public {
	fn verify_with_icon(
		&self,
		_icon_wallet: &types::IconAddress,
		_icon_signature: &[u8],
		_message: &[u8],
	) -> Result<(), types::SignatureValidationError> {
		Ok(())
	}
}

// Build genesis storage according to the mock runtime.
pub fn minimal_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

pub fn offchain_test_ext() -> (
	sp_io::TestExternalities,
	std::sync::Arc<parking_lot::RwLock<sp_core::offchain::testing::OffchainState>>,
	std::sync::Arc<parking_lot::RwLock<sp_core::offchain::testing::PoolState>>,
	<Test as frame_system::offchain::SigningTypes>::Public,
) {
	use sp_core::offchain::TransactionPoolExt;
	use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
	use sp_runtime::RuntimeAppPublic;
	use std::sync::Arc;

	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
	let keystore = KeyStore::new();
	let public_key = SyncCryptoStore::sr25519_generate_new(
		&keystore,
		crate::airdrop_crypto::Public::ID,
		Some(&format!("{}/abcdefg", PHRASE)),
	)
	.unwrap();

	let mut test_ext = sp_io::TestExternalities::default();
	let (pool, pool_state) = sp_core::offchain::testing::TestTransactionPoolExt::new();
	let (offchain, offchain_state) = sp_core::offchain::testing::TestOffchainExt::new();

	test_ext.register_extension(sp_core::offchain::OffchainWorkerExt::new(offchain));
	test_ext.register_extension(TransactionPoolExt::new(pool));
	test_ext.register_extension(KeystoreExt(Arc::new(keystore)));

	(test_ext, offchain_state, pool_state, public_key)
}

// Return the same address if it is not sudo
pub fn not_offchain_account(account: types::AccountIdOf<Test>) -> types::AccountIdOf<Test> {
	if account != AirdropModule::get_offchain_account().unwrap_or_default() {
		account
	} else {
		panic!("This address must not be same as defined in offchian worker. Change test value.");
	}
}

pub fn run_to_block(n: types::BlockNumberOf<Test>) {
	use frame_support::traits::Hooks;

	while System::block_number() < n {
		if System::block_number() > 1 {
			AirdropModule::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		AirdropModule::on_initialize(System::block_number());
	}
}

use sp_core::offchain::testing;
pub fn put_response(
	state: &mut testing::OffchainState,
	icon_address: &types::IconAddress,
	expected_response: &str,
) {
	let uri = String::from_utf8(
		mock::FetchIconEndpoint::get()
			.as_bytes()
			.iter()
			.chain(bytes::to_hex(icon_address, false).as_bytes().iter())
			.cloned()
			.collect::<Vec<u8>>(),
	)
	.unwrap();

	let response = if expected_response.is_empty() {
		None
	} else {
		Some(expected_response.to_string().as_bytes().to_vec())
	};

	state.expect_request(testing::PendingRequest {
		method: "GET".to_string(),
		uri,
		response,
		sent: true,
		..Default::default()
	});
}

pub fn get_last_event() -> Option<<Test as frame_system::Config>::Event> {
	<frame_system::Pallet<Test>>::events()
		.pop()
		.map(|v| v.event)
}

pub fn assert_tx_call(expected_call: &[&PalletCall], pool_state: &testing::PoolState) {
	use codec::Encode;

	let all_calls_in_pool = &pool_state.transactions;
	let expected_call_encoded = expected_call
		.iter()
		.map(|call| call.encode())
		.collect::<Vec<_>>();
	let all_calls_in_pool = all_calls_in_pool
		.iter()
		.enumerate()
		.map(|(index, call)| &call[call.len() - expected_call_encoded[index].len()..])
		.collect::<Vec<_>>();

	assert_eq!(expected_call_encoded, all_calls_in_pool);
}
