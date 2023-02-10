use cumulus_primitives_core::ParaId;
use frame_support::traits::GenesisBuild;
use polkadot_primitives::v2::{BlockNumber, MAX_CODE_SIZE, MAX_POV_SIZE};
use polkadot_runtime_parachains::configuration::HostConfiguration;
use sp_runtime::traits::AccountIdConversion;
use xcm_emulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};
use frame_support::pallet_prelude::Weight;
use crate::{dollar, get_all_module_accounts, ALICE, BOB, INITIAL_BALANCE, relay, para};

decl_test_relay_chain! {
	pub struct TestRelay {
		Runtime = relay::Runtime,
		XcmConfig = relay::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_parachain! {
	pub struct Arctic {
		Runtime = arctic_runtime::Runtime,
		XcmpMessageHandler = arctic_runtime ::XcmpQueue,
		DmpMessageHandler = arctic_runtime::DmpQueue,
		new_ext = para_ext(2001),
	}
}

decl_test_parachain! {
	pub struct Sibling {
		Runtime = arctic_runtime::Runtime,
		XcmpMessageHandler = arctic_runtime ::XcmpQueue,
		DmpMessageHandler = arctic_runtime::DmpQueue,
		new_ext = para_ext(2000),
	}
}

decl_test_network! {
	pub struct TestRelayWithArcticParaNet {
		relay_chain = TestRelay,
		parachains = vec![
			(2001, Arctic),
			(2000, Sibling),
		],
	}
}

pub fn relay_ext() -> sp_io::TestExternalities {
    use relay::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE), (ParaId::from(2001).into_account_truncating(), INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
    use arctic_runtime::{Runtime, System};

    let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

