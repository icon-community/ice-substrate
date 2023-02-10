
use frame_support::assert_ok;
use orml_traits::MultiCurrency;
// use xcm::{VersionedMultiLocation, v0::{MultiLocation::X1, Junction::{Parachain, self}}, VersionedMultiAssets};
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation};
use xcm_emulator::{Junctions::Here, NetworkId, TestExt};
use sp_runtime::traits::AccountIdConversion;
use crate::{
	dollar,
	testrelaywithtestpara_testnet::{TestRelay, Sibling, TestRelayTestParaANet, TestParaA},
	ALICE, BOB, INITIAL_BALANCE, RelayChainPalletXcm, ParachainPalletXcm,
    relay, para
};
use crate::para_account_id;


#[test]
fn reserve_transfer() {
    TestRelayTestParaANet::reset();

    let withdraw_amount = 123;

    TestRelay::execute_with(|| {
        assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
            relay::RuntimeOrigin::signed(ALICE),
            Box::new(X1(Parachain(2001)).into().into()),
            Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
            Box::new((Here, withdraw_amount).into()),
            0,
        ));
        assert_eq!(
            para::Balances::free_balance(&para_account_id(2001)),
            INITIAL_BALANCE + withdraw_amount
        );
    });

    TestParaA::execute_with(|| {
        // free execution, full amount received
        assert_eq!(
            pallet_balances::Pallet::<para::Runtime>::free_balance(&ALICE),
            INITIAL_BALANCE + withdraw_amount
        );
    });
}
