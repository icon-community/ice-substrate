use codec::Encode;
use frame_support::{traits::ConstU32, BoundedVec};

use crate::tests::to_test_case;

use super::prelude::*;
const VALID_ICON_WALLET: types::IconAddress =
	decode_hex!("ee1448f0867b90e6589289a4b9c06ac4516a75a9");

#[test]
fn claim_success() {
	let sample = samples::MERKLE_PROOF_SAMPLE;
	let case = to_test_case(sample);
	let bounded_proofs = BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
	let defi_user = true;
	let amount: types::BalanceOf<Test> = 10017332_u64.into();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");
		let ice_address = AirdropModule::convert_to_account_id(ice_address.clone()).unwrap();

		pallet_airdrop::ExchangeAccountsMap::<Test>::insert(icon_wallet, amount);
		set_creditor_balance(10_000_0000);

		assert_ok!(AirdropModule::dispatch_exchange_claim(
			RuntimeOrigin::root(),
			icon_wallet,
			ice_address.encode().try_into().unwrap(),
			amount.into(),
			defi_user,
			bounded_proofs,
		));

		let snapshot = AirdropModule::get_icon_snapshot_map(&icon_wallet).unwrap();
		let expected_vesting_block_number = if cfg!(feature = "no-vesting") {
			None
		} else {
			Some(0)
		};

		// Ensure mapping in both storage are correct
		let mapped_icon_wallet = AirdropModule::get_ice_to_icon_map(&ice_address);
		assert_eq!(mapped_icon_wallet, Some(icon_wallet));

		// Ensure transfer flag are updated
		assert!(snapshot.done_instant);
		assert_eq!(Some(0), snapshot.instant_block_number);
		assert!(snapshot.done_vesting);
		assert_eq!(expected_vesting_block_number, snapshot.vesting_block_number);
	});
}

#[test]
fn insufficient_balance() {
	let sample = samples::MERKLE_PROOF_SAMPLE;
	let case = to_test_case(sample);
	let bounded_proofs = BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
	let defi_user = true;
	let amount: types::BalanceOf<Test> = 10017332_u64.into();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");

		let creditor_account = force_get_creditor_account::<Test>();
		pallet_airdrop::ExchangeAccountsMap::<Test>::insert(&icon_wallet, amount);
		<Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::RuntimeOrigin::root(),
			creditor_account,
			10_u32.into(),
			10_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_exchange_claim(
				RuntimeOrigin::root(),
				icon_wallet,
				ice_address.clone(),
				amount,
				defi_user,
				bounded_proofs,
			),
			PalletError::InsufficientCreditorBalance
		);
	});
}

#[test]
fn double_exchange() {
	let exchange_once = || {
		set_creditor_balance(10_00_00_0000);

		let case = to_test_case(samples::MERKLE_PROOF_SAMPLE);
		let bounded_proofs =
			BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
		let amount: types::BalanceOf<Test> = 10017332_u64.into();
		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");
		pallet_airdrop::ExchangeAccountsMap::<Test>::insert(&icon_wallet, amount);

		AirdropModule::dispatch_exchange_claim(
			RuntimeOrigin::root(),
			icon_wallet,
			ice_address.clone(),
			amount,
			true,
			bounded_proofs,
		)
	};

	minimal_test_ext().execute_with(|| {
		let first_exchange = exchange_once();
		let second_exchange = exchange_once();

		// First exchange should pass
		assert_ok!(first_exchange);

		// Second exchange should fail
		assert_err!(second_exchange, PalletError::ClaimAlreadyMade);
	});
}

#[test]
fn already_claimed() {
	let sample = samples::MERKLE_PROOF_SAMPLE;
	let case = to_test_case(sample);
	let bounded_proofs = BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
	let defi_user = true;
	let amount: types::BalanceOf<Test> = 1;

	minimal_test_ext().execute_with(|| {
		set_creditor_balance(10_000_0000_u32.into());

		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");
		let ice_account = AirdropModule::convert_to_account_id(ice_address).unwrap();

		let mut snapshot = types::SnapshotInfo::default().ice_address(ice_account.clone());
		snapshot.done_instant = true;
		snapshot.done_vesting = true;

		pallet_airdrop::IconSnapshotMap::<Test>::insert(&icon_wallet, snapshot);
		pallet_airdrop::ExchangeAccountsMap::<Test>::insert(&icon_wallet, amount);

		assert_err!(
			AirdropModule::dispatch_exchange_claim(
				RuntimeOrigin::root(),
				icon_wallet,
				ice_address.clone(),
				amount,
				defi_user,
				bounded_proofs,
			),
			PalletError::ClaimAlreadyMade
		);
	});
}

#[test]
fn only_whitelisted_claim() {
	let sample = samples::MERKLE_PROOF_SAMPLE;
	let case = to_test_case(sample);
	let bounded_proofs = BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
	let defi_user = true;
	let amount: types::BalanceOf<Test> = 10017332_u64.into();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");

		let snapshot = types::SnapshotInfo::default();

		pallet_airdrop::IconSnapshotMap::<Test>::insert(&icon_wallet, snapshot);
		let creditor_account = force_get_creditor_account::<Test>();
		<Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::RuntimeOrigin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_exchange_claim(
				RuntimeOrigin::root(),
				icon_wallet,
				ice_address.clone(),
				amount,
				defi_user,
				bounded_proofs,
			),
			PalletError::DeniedOperation
		);
	});
}

#[test]
fn invalid_claim_amount() {
	let sample = samples::MERKLE_PROOF_SAMPLE;
	let case = to_test_case(sample);
	let bounded_proofs = BoundedVec::<types::MerkleHash, ConstU32<10>>::try_from(case.1).unwrap();
	let defi_user = true;
	let amount: types::BalanceOf<Test> = 10017332_u64.into();
	let mut test_ext = minimal_test_ext();
	test_ext.execute_with(|| {
		let icon_wallet = VALID_ICON_WALLET;
		let ice_address =
			hex_literal::hex!("da8db20713c087e12abae13f522693299b9de1b70ff0464caa5d392396a8f76c");

		let mut snapshot = types::SnapshotInfo::default();
		snapshot.done_instant = true;
		snapshot.done_vesting = true;

		pallet_airdrop::IconSnapshotMap::<Test>::insert(&icon_wallet, snapshot);
		let creditor_account = force_get_creditor_account::<Test>();
		pallet_airdrop::ExchangeAccountsMap::<Test>::insert(&icon_wallet, amount);
		<Test as pallet_airdrop::Config>::Currency::set_balance(
			mock::RuntimeOrigin::root(),
			creditor_account,
			10_000_0000_u32.into(),
			10_000_00_u32.into(),
		)
		.unwrap();

		assert_err!(
			AirdropModule::dispatch_exchange_claim(
				RuntimeOrigin::root(),
				icon_wallet,
				ice_address.clone(),
				amount + 10000,
				defi_user,
				bounded_proofs,
			),
			PalletError::InvalidClaimAmount
		);
	});
}
