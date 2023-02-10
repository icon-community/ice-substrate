use arctic_runtime::{
	AccountId, Balances, CurrencyId, RuntimeOrigin, RelayCurrencyId, TokenSymbol, Tokens, XTokens,
};
use frame_support::assert_ok;
use orml_traits::MultiCurrency;
// use xcm::{VersionedMultiLocation, v0::{MultiLocation::X1, Junction::{Parachain, self}}, VersionedMultiAssets};
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation};
use xcm_emulator::{Junctions::Here, NetworkId, TestExt};
use sp_runtime::traits::AccountIdConversion;
use crate::{
	dollar,
	rococo_testnet::{Arctic, Rococo, Sibling, RococoNet},
	ALICE, BOB, INITIAL_BALANCE, RelayChainPalletXcm, ParachainPalletXcm,
    relay, para
};
use polkadot_parachain::primitives::Id as ParaId;

#[test]
fn transfer_from_relay_chain() {
    RococoNet::reset();
    let withdraw_amt = 123;

	Rococo::execute_with(|| {

		assert_ok!(// rococo_runtime::XcmPallet::reserve_transfer_assets(
            RelayChainPalletXcm::reserve_transfer_assets(
			// rococo_runtime::RuntimeOrigin::signed(ALICE.into()),
			relay::RuntimeOrigin::signed(ALICE.into()),
			// Box::new(VersionedMultiLocation::V1(X1(Parachain(2001)).into())),
            Box::new(Parachain(2001).into().into()),
			Box::new(VersionedMultiLocation::V1(
				X1(Junction::AccountId32 {
					id: ALICE.into(),
					network: NetworkId::Any
				})
				.into()
			)),
			Box::new(VersionedMultiAssets::V1(
				// (Here, dollar(RelayCurrencyId::get())).into()
                (Here, withdraw_amt).into()
			)),
			0,
		));
        assert_eq!(
            Balances::free_balance(&ParaId::from(2001).into_account_truncating()),
            INITIAL_BALANCE + withdraw_amt
        );
	});

	// Arctic::execute_with(|| {
    Arctic::execute_with(|| {
		assert_eq!(
			// Tokens::free_balance(RelayCurrencyId::get(), &AccountId::from(BOB)),
        // Balances::free_balance(&AccountId::from(ALICE)),
        // pallet_balances::Pallet::<arctic_runtime::Runtime>::free_balance(&ALICE),
        pallet_balances::Pallet::<para::Runtime>::free_balance(&ALICE),
        INITIAL_BALANCE + withdraw_amt
		);
	});
}



#[test]
fn transfer_to_relay_chain() {
	Arctic::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			RelayCurrencyId::get(),
			dollar(RelayCurrencyId::get()),
			Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				})
			))),
			// 4_000_000_000
            WeightLimit::Unlimited

		));
	});

	Rococo::execute_with(|| {
		assert_eq!(
			rococo_runtime::Balances::free_balance(&AccountId::from(BOB)),
			999_834_059_328
		);
	});
}

#[test]
fn transfer_to_sibling() {
	env_logger::init();

	RococoNet::reset();

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
			RuntimeOrigin::signed(ALICE.into()),
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
			// 1_000_000_000,
            WeightLimit::Unlimited
		));

		assert_eq!(
			Tokens::free_balance(CurrencyId::Token(TokenSymbol::KSM), &AccountId::from(ALICE)),
			90_000_000_000_000
		);
	});
}
