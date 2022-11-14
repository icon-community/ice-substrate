mod exchange_claim;
mod merkle_tests;
pub mod mock;
mod signature_validation;
mod user_claim;
mod utility_functions;
pub mod prelude {
	pub use super::{
		force_get_creditor_account, get_last_event, minimal_test_ext, mock, run_to_block, samples,
		set_creditor_balance, transfer_to_creditor,
	};
	pub use crate as pallet_airdrop;
	pub use codec::Encode;
	pub use frame_support::{
		assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_noop, assert_ok,
		assert_storage_noop,
	};
	pub use hex_literal::hex as decode_hex;
	pub use mock::{AirdropModule, RuntimeOrigin, Test};
	pub use pallet_airdrop::{tests, transfer, types, utils};
	pub use sp_core::bytes;
	pub use sp_runtime::traits::{Bounded, IdentifyAccount, Saturating};

	pub type PalletError = pallet_airdrop::Error<Test>;
	pub type PalletEvent = pallet_airdrop::Event<Test>;
	pub type PalletCall = pallet_airdrop::Call<Test>;
	pub type BalanceError = pallet_balances::Error<Test>;
}

use frame_support::{traits::ConstU32, BoundedVec};
use mock::System;
use prelude::*;

#[derive(Clone)]
pub struct UserClaimTestCase {
	pub icon_address: [u8; 20],
	pub ice_address: types::IceAddress,
	pub message: types::RawPayload,
	pub icon_signature: [u8; 65],
	pub ice_signature: [u8; 64],
	pub amount: u128,
	pub defi_user: bool,
	pub merkle_proofs: BoundedVec<types::MerkleHash, ConstU32<10>>,
	pub merkle_root: [u8; 32],
}

impl Default for UserClaimTestCase {
	fn default() -> Self {
		let (root, proofs) = to_test_case(samples::MERKLE_PROOF_SAMPLE);
		let bounded_proofs =
			BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(proofs).unwrap();
		Self {
			icon_address: samples::VALID_ICON_WALLET,
			ice_address: samples::VALID_ICE_ADDRESS,
			message: samples::VALID_MESSAGE,
			icon_signature: samples::VALID_ICON_SIGNATURE,
			ice_signature: samples::VALID_ICE_SIGNATURE,
			amount: 12_000_000,
			defi_user: true,
			merkle_proofs: bounded_proofs,
			merkle_root: root,
		}
	}
}

pub mod samples {

	use super::decode_hex;
	use super::types::{IconAddress, IconSignature, RawPayload};
	use sp_core::sr25519;

	pub const ACCOUNT_ID: &[sr25519::Public] = &[
		sr25519::Public([1; 32]),
		sr25519::Public([2; 32]),
		sr25519::Public([3; 32]),
		sr25519::Public([4; 32]),
		sr25519::Public([5; 32]),
	];

	pub const ICON_ADDRESS: &[IconAddress] = &[
		decode_hex!("ee1448f0867b90e6589289a4b9c06ac4516a75a9"),
		decode_hex!("ee33286f367b90e6589289a4b987a6c4516a753a"),
		decode_hex!("ee12463586abb90e6589289a4b9c06ac4516a7ba"),
		decode_hex!("ee02363546bcc50e643910104321c0623451a65a"),
	];

	pub const MERKLE_PROOF_SAMPLE: (&str, &[&str]) = (
		"7fe522d63ebcabfa052eec3647366138c23c9870995f4af94d9b22b8c5923f49",
		&[
			"813340daefd7f1ca705faf8318cf6455632259d113c06e97b70eeeccd43519a9",
			"409519ab7129397bdc895e4da05871c9725697a5e092addf2fe90f6e795feb8f",
			"38055bb872670c69ac3461707f8c0b4b8e436eecfc84cfd80db30db3030c489a",
		],
	);

	pub const VALID_ICON_SIGNATURE:IconSignature = decode_hex!("9ee3f663175691ad82f4fbb0cfd0594652e3a034e3b6934b0e4d4a60437ba4043c89d2ffcb7b0af49ed0744ce773612d7ebcdf3a5b035c247706050e0a0033e401");
	pub const VALID_MESSAGE: RawPayload = *b"icx_sendTransaction.data.{method.transfer.params.{wallet.b6e7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48}}.dataType.call.from.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hxb48f3bd3862d4a489fb3c9b761c4cfb20b34a645.version.0x3";
	pub const VALID_ICON_WALLET: IconAddress =
		decode_hex!("b48f3bd3862d4a489fb3c9b761c4cfb20b34a645");
	pub const VALID_ICE_ADDRESS: [u8; 32] =
		decode_hex!("b6e7a79d04e11a2dd43399f677878522523327cae2691b6cd1eb972b5a88eb48");
	pub const VALID_ICE_SIGNATURE : [u8;64] =decode_hex!("901bda07fb98882a4944f50925b45d041a8a05751a45501eab779416bb55ca5537276dad3c68627a7ddb96956a17ae0d89ca27901a9638ad26426d0e2fbf7e8a");
}

// Build genesis storage according to the mock runtime.
pub fn minimal_test_ext() -> sp_io::TestExternalities {
	use codec::Decode;
	use frame_support::traits::GenesisBuild;
	use hex_literal::hex;
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	let account_hex = hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"];
	let account_id = types::AccountIdOf::<Test>::decode(&mut &account_hex[..]).unwrap();
	pallet_airdrop::GenesisConfig::<Test> {
		creditor_account: account_id,
		merkle_root: hex!["4c59b428da385567a6d42ee1881ecbe43cf30bf8c4499887b7c6f689d23d4672"],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
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

pub fn get_last_event() -> Option<<Test as frame_system::Config>::RuntimeEvent> {
	<frame_system::Pallet<Test>>::events()
		.pop()
		.map(|v| v.event)
}

pub fn set_creditor_balance(balance: u64) {
	let creditor_account = force_get_creditor_account::<Test>();
	let deposit_res = <Test as pallet_airdrop::Config>::Currency::set_balance(
		mock::RuntimeOrigin::root(),
		creditor_account,
		balance.into(),
		0u32.into(),
	);

	assert_ok!(deposit_res);
	assert_eq!(
		<Test as pallet_airdrop::Config>::Currency::free_balance(&creditor_account),
		balance.into()
	);
}

pub fn to_test_case(
	sample: (&str, &'static [&str]),
) -> (types::MerkleHash, Vec<types::MerkleHash>) {
	let mut hash_bytes = [0u8; 32];
	hex::decode_to_slice(sample.0, &mut hash_bytes as &mut [u8]).unwrap();
	let proofs = sample
		.1
		.iter()
		.map(|p| {
			let mut bytes: [u8; 32] = [0u8; 32];
			hex::decode_to_slice(p, &mut bytes as &mut [u8]).unwrap();
			bytes
		})
		.collect();

	(hash_bytes, proofs)
}

pub fn transfer_to_creditor(sponsor: &types::AccountIdOf<Test>, amount: types::BalanceOf<Test>) {
	assert_ok!(<Test as pallet_airdrop::Config>::Currency::transfer(
		RuntimeOrigin::signed(sponsor.clone()),
		force_get_creditor_account::<Test>(),
		amount,
	));
}

pub fn force_get_creditor_account<T: pallet_airdrop::Config>() -> types::AccountIdOf<T> {
	pallet_airdrop::Pallet::<T>::get_creditor_account().expect("creditor account not set")
}

impl Default for types::SnapshotInfo<Test> {
	fn default() -> Self {
		Self::new(
			types::AccountIdOf::<Test>::from_raw([0u8; 32]),
			false,
			0u32.into(),
		)
	}
}

impl<T> types::SnapshotInfo<T>
where
	T: pallet_airdrop::Config,
{
	pub fn ice_address(mut self, val: types::AccountIdOf<T>) -> Self {
		self.ice_address = val;
		self
	}
}
