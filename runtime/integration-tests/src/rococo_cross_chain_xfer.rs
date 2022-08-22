use arctic_runtime::{Tokens, RelayCurrencyId, AccountId, XTokens, Origin};
use frame_support::assert_ok;
use orml_traits::MultiCurrency;
// use xcm::{VersionedMultiLocation, v0::{MultiLocation::X1, Junction::{Parachain, self}}, VersionedMultiAssets};
use xcm::{latest::prelude::*, VersionedMultiAssets, VersionedMultiLocation};
use xcm_emulator::{TestExt, NetworkId, Junctions::Here};

use crate::{BOB, rococo_testnet::{RococoNet, Arctic}, ALICE, dollar};

#[test]
fn transfer_from_relay_chain() {
	RococoNet::execute_with(|| {
		assert_ok!(rococo_runtime::XcmPallet::reserve_transfer_assets(
			rococo_runtime::Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V1(X1(Parachain(2001)).into())),
			Box::new(VersionedMultiLocation::V1(
				X1(Junction::AccountId32 { id: BOB, network: NetworkId::Any }).into()
			)),
			Box::new(VersionedMultiAssets::V1((Here, dollar(RelayCurrencyId::get())).into())),
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
				X1(Junction::AccountId32 { id: BOB, network: NetworkId::Any })
			))),
			4_000_000_000
		));
	});

	RococoNet::execute_with(|| {
		assert_eq!(kusama_runtime::Balances::free_balance(&AccountId::from(BOB)), 999_834_059_328);
	});
}
