use xcm_emulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

decl_test_relay_chain! {
	pub struct KusamaNet {
		Runtime = kusama_runtime::Runtime,
		XcmConfig = kusama_runtime::xcm_config::XcmConfig,
		new_ext = kusama_ext(),
	}
}

decl_test_parachain! {
	pub struct Arctic {
		Runtime = Runtime,
		Origin = Origin,
		XcmpMessageHandler = arctic_runtime::XcmpQueue,
		DmpMessageHandler = arctic_runtime::DmpQueue,
		new_ext = para_ext(2000),
	}
}

decl_test_parachain! {
	pub struct MockBifrost {
		Runtime = Runtime,
		Origin = Origin,
		XcmpMessageHandler = arctic_runtime::XcmpQueue,
		DmpMessageHandler = arctic_runtime::DmpQueue,
		new_ext = para_ext(2001),
	}
}

decl_test_parachain! {
	pub struct Sibling {
		Runtime = Runtime,
		Origin = Origin,
		XcmpMessageHandler = arctic_runtime::XcmpQueue,
		DmpMessageHandler = arctic_runtime::DmpQueue,
		new_ext = para_ext(2002),
	}
}

decl_test_parachain! {
	pub struct Statemine {
		Runtime = statemine_runtime::Runtime,
		Origin = statemine_runtime::Origin,
		XcmpMessageHandler = statemine_runtime::XcmpQueue,
		DmpMessageHandler = statemine_runtime::DmpQueue,
		new_ext = para_ext(1000),
	}
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = KusamaNet,
		parachains = vec![
			(1000, Statemine),
			(2000, Karura),
			(2001, MockBifrost),
			(2002, Sibling),
		],
	}
}