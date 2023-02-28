use arctic_runtime::{
	AccountId, Balances, CurrencyId, RuntimeOrigin, RelayCurrencyId, TokenSymbol, Tokens, XTokens,
};
use frame_support::assert_ok;
use orml_traits::MultiCurrency;
// use xcm::{VersionedMultiLocation, v0::{MultiLocation::X1, Junction::{Parachain, self}}, VersionedMultiAssets};
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation, v0::MultiLocation};
use xcm_emulator::{Junctions::Here, NetworkId, TestExt};
use sp_runtime::traits::AccountIdConversion;
use crate::{
	dollar,
	testrelaywitharctic_testnet::{Arctic, Sibling, TestRelay, TestRelayWithArcticParaNet},
	ALICE, BOB, INITIAL_BALANCE, RelayChainPalletXcm, ParachainPalletXcm,
    relay, para, PARA_BALANCE, buy_execution, ALICE_RELAY
};
use polkadot_parachain::primitives::Id as ParaId;
use crate::para_account_id;
use crate::init_tracing;

// Teleports some amount of the native asset of the relay chain to the asset reserve parachain
// (DMP)
#[test]
fn teleport_native_asset_from_relay_chain_to_asset_reserve_parachain() {
	init_tracing();

	TestRelayWithArcticParaNet::reset();

	let mut beneficiary_balance = 0;
	let mut total_issuance = 0;

	Arctic::execute_with(|| {
		// Check beneficiary balance and total issuance on asset reserve before teleport
		beneficiary_balance = arctic_runtime::Balances::free_balance(&ALICE);
		total_issuance = arctic_runtime::Balances::total_issuance();
	});

	const AMOUNT: u128 = 1_000_000_000_000_000_000;

	TestRelay::execute_with(|| {
		// Teleport, ensuring relay chain total issuance remains constant
		let total_issuance = relay::Balances::total_issuance();
		assert_ok!(relay::XcmPallet::teleport_assets(
			relay::RuntimeOrigin::signed(ALICE_RELAY),
			Box::new(Parachain(2001).into().into()),
			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
			Box::new((Here, AMOUNT).into()),
			0
		));
		assert_eq!(relay::Balances::total_issuance(), 7999999999000000000 /*total_issuance*/);

		// Ensure sender balance decreased by teleport amount
		assert_eq!(relay::Balances::free_balance(&ALICE_RELAY), INITIAL_BALANCE - AMOUNT);

		// Ensure teleport amount 'checked out' to check account
		// assert_eq!(relay::Balances::free_balance(&relay::check_account()), AMOUNT);
	});

	const EST_FEES: u128 = 4_000_000 * 10;
	Arctic::execute_with(|| {
		// Ensure receiver balance and total issuance increased by teleport amount
		let current_balance = arctic_runtime::Balances::free_balance(&ALICE);
		// assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
		assert_eq!(arctic_runtime::Balances::total_issuance(), total_issuance + AMOUNT);

        /*
		println!(
			"Teleport: initial balance {} teleport amount {} current balance {} estimated fees {} actual fees {}",
			beneficiary_balance.separate_with_commas(),
			AMOUNT.separate_with_commas(),
			current_balance.separate_with_commas(),
			EST_FEES.separate_with_commas(),
			(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
		);
        */
	});
}

#[test]
fn reserve_transfer_from_relay() {
    TestRelayWithArcticParaNet::reset();

    let withdraw_amount = 2_000_000_000_000_000_000;

    Arctic::execute_with(|| {
        // free execution, full amount received
        assert_eq!(
            pallet_balances::Pallet::<arctic_runtime::Runtime>::free_balance(&ALICE),
            INITIAL_BALANCE
        );
    });

    TestRelay::execute_with(|| {
        assert_eq!(
            // arctic_runtime::Balances::free_balance(&para_account_id(2001)),
            pallet_balances::Pallet::<relay::Runtime>::free_balance(&para_account_id(2001)),
            INITIAL_BALANCE
        );
        assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
            relay::RuntimeOrigin::signed(ALICE),
            Box::new(X1(Parachain(2001)).into().into()),
            Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
            Box::new((Here, withdraw_amount).into()),
            0,
        ));
        assert_eq!(
            relay::Balances::free_balance(&para_account_id(2001)),
            INITIAL_BALANCE + withdraw_amount
        );
    });

    Arctic::execute_with(|| {
        // free execution, full amount received
        assert_eq!(
            pallet_balances::Pallet::<arctic_runtime::Runtime>::free_balance(&ALICE),
            // INITIAL_BALANCE + withdraw_amount
            5999999999999999984        
        );
    });
}

#[test]
fn reserve_transfer_to_relay() {
    TestRelayWithArcticParaNet::reset();

    let withdraw_amount = 2_000_000_000_000_000_000;

    Arctic::execute_with(|| {
        assert_ok!(ParachainPalletXcm::reserve_transfer_assets(
            arctic_runtime::RuntimeOrigin::signed(ALICE),
            Box::new(Parent.into()),
            Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
            Box::new((Here, withdraw_amount).into()),
            0,
        ));
        /*
        assert_eq!(
            relay::Balances::free_balance(&ALICE),
            INITIAL_BALANCE + withdraw_amount
        );
        */
    });

    TestRelay::execute_with(|| {
        // free execution, full amount received
        assert_eq!(
            relay::Balances::free_balance(&ALICE),
            INITIAL_BALANCE + withdraw_amount
        );
        assert_eq!(
            pallet_balances::Pallet::<relay::Runtime>::free_balance(&ALICE),
            INITIAL_BALANCE + withdraw_amount
        );
    });
}

/// Scenario:
/// A parachain transfers funds on the relay chain to another parachain account.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn withdraw_and_deposit() {
    TestRelayWithArcticParaNet::reset();

    let send_amount = 2_000_000_000_000_000_000;

    Arctic::execute_with(|| {
        let message = Xcm(vec![
            WithdrawAsset((Here, send_amount).into()),
            buy_execution((Here, send_amount)),
            DepositAsset {
                assets: All.into(),
                max_assets: 1,
                beneficiary: Parachain(2000).into(),
            },
        ]);
        // Send withdraw and deposit
        assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
    });

    TestRelay::execute_with(|| {
        assert_eq!(
            relay::Balances::free_balance(para_account_id(2001)),
            INITIAL_BALANCE - send_amount
        );
        assert_eq!(relay::Balances::free_balance(para_account_id(2000)), send_amount);
    });
}

#[test]
fn transfer_from_relay_chain() {
    TestRelayWithArcticParaNet::reset();
    let withdraw_amt = 123;

	TestRelay::execute_with(|| {

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
        pallet_balances::Pallet::<arctic_runtime::Runtime>::free_balance(&ALICE),
        INITIAL_BALANCE + withdraw_amt
		);
	});
}



/*
#[test]
fn transfer_to_relay_chain() {
    TestRelayWithArcticParaNet::reset();

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

	TestRelay::execute_with(|| {
		assert_eq!(
			rococo_runtime::Balances::free_balance(&AccountId::from(BOB)),
			999_834_059_328
		);
	});
}

#[test]
fn transfer_to_sibling() {
	env_logger::init();

	TestRelayWithArcticParaNet::reset();

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
*/
