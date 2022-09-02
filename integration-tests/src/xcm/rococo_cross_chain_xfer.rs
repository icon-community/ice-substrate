use arctic_runtime::{
	AccountId, CurrencyId, Origin, RelayCurrencyId, TokenSymbol, Tokens, XTokens, AssetRegistry, Balances,
};
use frame_support::assert_ok;
use orml_asset_registry::AssetMetadata;
use orml_traits::MultiCurrency;
// use xcm::{VersionedMultiLocation, v0::{MultiLocation::X1, Junction::{Parachain, self}}, VersionedMultiAssets};
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation};
use xcm_emulator::{Junctions::Here, NetworkId, TestExt};

use crate::{
	xcm::dollar,
	xcm::rococo_testnet::{Arctic, RococoNet, Sibling, TestNet},
	xcm::ALICE, xcm::BOB,
};

#[test]
fn transfer_from_relay_chain() {
	RococoNet::execute_with(|| {
		assert_ok!(rococo_runtime::XcmPallet::reserve_transfer_assets(
			rococo_runtime::Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V1(X1(Parachain(2001)).into())),
			Box::new(VersionedMultiLocation::V1(
				X1(Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				})
				.into()
			)),
			Box::new(VersionedMultiAssets::V1(
				(Here, dollar(RelayCurrencyId::get())).into()
			)),
			0,
		));
	});

	Arctic::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(RelayCurrencyId::get(), &AccountId::from(BOB)),
			999936000000
		);
	});
}

#[test]
fn transfer_to_relay_chain() {
	Arctic::execute_with(|| {
		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			RelayCurrencyId::get(),
			dollar(RelayCurrencyId::get()),
			Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				})
			))),
			4_000_000_000
		));
	});

	RococoNet::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
			999_834_059_328
		);
	});
}

#[test]
fn transfer_to_sibling() {
	env_logger::init();

	TestNet::reset();

	fn arctic_reserve_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(2001).into_account_truncating()
	}

	Arctic::execute_with(|| {
		assert_ok!(Tokens::deposit(
			CurrencyId::Token(TokenSymbol::KSM),
			&AccountId::from(ALICE),
			100_000_000_000_000
		));
	});

	Sibling::execute_with(|| {
		assert_ok!(Tokens::deposit(
			CurrencyId::Token(TokenSymbol::KSM),
			&arctic_reserve_account(),
			100_000_000_000_000
		));
	});

	Arctic::execute_with(|| {
		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			CurrencyId::Token(TokenSymbol::KSM),
			10_000_000_000_000,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(2000),
						Junction::AccountId32 {
							network: NetworkId::Any,
							id: BOB.into()
						}
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(
			Tokens::free_balance(CurrencyId::Token(TokenSymbol::KSM), &AccountId::from(ALICE)),
			90_000_000_000_000
		);
	});
}

/*
#[test]
fn transfer_native_chain_asset() {
	TestNet::reset();
    pub const BNC_KEY: &[u8] = &[0, 1];

    enum CurrencyId {
        N
    }

	let dollar = 10u128.saturating_pow(currency_id.decimals().unwrap_or(12).into()); // dollar(BNC);
	let minimal_balance = Balances::minimum_balance() / 10; // 10%
	let foreign_fee = foreign_per_second_as_fee(4, minimal_balance);
	let bnc_fee = bnc_per_second_as_fee(4);

	Sibling::execute_with(|| {
		// Register native BNC's incoming address as a foreign asset so it can receive BNC
		assert_ok!(AssetRegistry::register_asset(
			Origin::root(),
			Box::new(MultiLocation::new(0, X1(GeneralKey(BNC_KEY.to_vec().try_into().unwrap()))).into()),
			Box::new(AssetMetadata {
				name: b"Native BNC".to_vec(),
				symbol: b"BNC".to_vec(),
				decimals: 12,
				minimal_balance
			})
		));
		assert_ok!(Tokens::deposit(
			CurrencyId::ForeignAsset(0),
			&karura_reserve_account(),
			100 * dollar
		));

		assert_ok!(Tokens::deposit(BNC, &AccountId::from(ALICE), 100 * dollar));

		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			BNC,
			10 * dollar,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(KARURA_ID),
						Junction::AccountId32 {
							network: NetworkId::Any,
							id: BOB.into(),
						}
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(ALICE)), 90 * dollar);
		assert_eq!(Tokens::free_balance(BNC, &karura_reserve_account()), 10 * dollar);
	});

	Arctic::execute_with(|| {
		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(BOB)), 10 * dollar - bnc_fee);

		assert_ok!(XTokens::transfer(
			Origin::signed(BOB.into()),
			BNC,
			5 * dollar,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(MOCK_BIFROST_ID),
						Junction::AccountId32 {
							network: NetworkId::Any,
							id: ALICE.into(),
						}
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(BOB)), 5 * dollar - bnc_fee);
	});

	Sibling::execute_with(|| {
		// Due to the re-anchoring, BNC came back as registered ForeignAsset(0)
		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(ALICE)), 90 * dollar);
		assert_eq!(Tokens::free_balance(BNC, &karura_reserve_account()), 10 * dollar);

		assert_eq!(
			Tokens::free_balance(CurrencyId::ForeignAsset(0), &AccountId::from(ALICE)),
			5 * dollar - foreign_fee
		);
		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(ALICE)), 90 * dollar);
	});
}

*/