use crate::mock;
// mod claim;
mod signature_validation;
mod transfer;
mod utility_functions;

pub mod prelude {
	pub use super::{
		get_last_event, minimal_test_ext, not_airdrop_sudo, offchain_test_ext, put_response,
		samples,
	};
	pub use crate as pallet_airdrop;
	pub use crate::tests;
	pub use frame_support::{
		assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_noop, assert_ok,
		assert_storage_noop,
	};
	pub use pallet_airdrop::mock::{self, AirdropModule, Origin, Test};
	pub use pallet_airdrop::types;
	pub type PalletError = pallet_airdrop::Error<Test>;
	pub type PalletEvent = pallet_airdrop::Event<Test>;
	pub use sp_core::bytes;
}
use mock::System;
use prelude::*;

pub mod samples {
	use super::types::{ServerResponse, SnapshotInfo};
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

	pub const ICON_ADDRESS: &[&str] = &[
		"0xee1448f0867b90e6589289a4b9c06ac4516a75a9",
		"0xee33286f367b90e6589289a4b987a6c4516a753a",
		"0xee12463586abb90e6589289a4b9c06ac4516a7ba",
		"0xee02363546bcc50e643910104321c0623451a65a",
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
) {
	use sp_core::offchain::TransactionPoolExt;
	use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
	use sp_runtime::RuntimeAppPublic;
	use std::sync::Arc;

	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
	let keystore = KeyStore::new();
	SyncCryptoStore::sr25519_generate_new(
		&keystore,
		crate::airdrop_crypto::Public::ID,
		Some(&format!("{}/abcdefg", PHRASE)),
	)
	.unwrap();

	let mut test_ext = sp_io::TestExternalities::default();
	let (pool, _pool_state) = sp_core::offchain::testing::TestTransactionPoolExt::new();
	let (offchain, state) = sp_core::offchain::testing::TestOffchainExt::new();

	test_ext.register_extension(sp_core::offchain::OffchainWorkerExt::new(offchain));
	test_ext.register_extension(TransactionPoolExt::new(pool));
	test_ext.register_extension(KeystoreExt(Arc::new(keystore)));

	(test_ext, state)
}

// Return the same address if it is not sudo
pub fn not_airdrop_sudo(account: types::AccountIdOf<Test>) -> types::AccountIdOf<Test> {
	if account != AirdropModule::get_sudo_account() {
		account
	} else {
		panic!("This address must not be sudo. Change test value.");
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
			.chain(icon_address.iter())
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
