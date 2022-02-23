use ice_runtime::{
	AccountId, AuraConfig, BalancesConfig, CouncilConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY, currency::ICY
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap};
use hex_literal::hex;
use std::marker::PhantomData;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ICE_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 18,
            "tokenSymbol": "ICZ"
        }"#;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Testnet",
		// ID
		"testnet",
		ChainType::Custom(String::from("snow")),
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						// AuraId
						hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].unchecked_into(),
						// GrandpaId
						hex!["27c6da25d03bb6b3c751da3e8c5265b0bb357c15240602443cc286c0658b47f9"].unchecked_into(),
					),
					(
						hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].unchecked_into(),
						hex!["85ec524aeacb6e558619a10da82cdf787026209211d1b7462cb176d58f2add86"].unchecked_into()
					)
				],
				// Council members
                vec![
                    hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into()
                ],
				// Sudo account
				hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
				// Pre-funded accounts
				vec![
					hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
					hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].into(),
					hex!["98003761bff94c8c44af38b8a92c1d5992d061d41f700c76255c810d447d613f"].into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		serde_json::from_str(ICE_PROPERTIES).unwrap(),
		// Extensions
		None,
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
                // Council members
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice")
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		serde_json::from_str(ICE_PROPERTIES).unwrap(),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Council members
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice")
                ],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	council_members: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ICY * 300_000_000))
				.collect()
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		evm: EVMConfig {
			accounts: {
				let map = BTreeMap::new();
				map
			},
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		vesting:Default::default(),
		assets: Default::default(),
        council: CouncilConfig {
            members: council_members,
            phantom: PhantomData,
        },
        treasury: Default::default(),
	}
}
