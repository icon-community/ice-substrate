#![allow(clippy::too_many_arguments)]

use frost_runtime::{
	currency::ICY, opaque::SessionKeys, AccountId, AirdropConfig, AssetsConfig, AuraConfig,
	BalancesConfig, CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig,
	EthereumConfig, GenesisConfig, GrandpaConfig, IndicesConfig, SS58Prefix, SessionConfig,
	Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig, TechnicalMembershipConfig,
	TreasuryPalletId, WASM_BINARY,
};
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{AccountIdConversion, IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type FrostChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn frost_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("tokenSymbol".into(), "ICY".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	properties
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

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex!("990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122");

/// Initialize frost development configuration
pub fn development_config() -> Result<FrostChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities = vec![authority_keys_from_seed("Alice")];

	let council_members = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let technical_committee_membership = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	let endowed_accounts = vec![
		TreasuryPalletId::get().into_account_truncating(),
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
	];

	Ok(FrostChainSpec::from_genesis(
		"Frost Development",
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				initial_authorities.clone(),
				council_members.clone(),
				technical_committee_membership.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				endowed_accounts.clone(),
				true,
			)
		},
		vec![],
		None,
		None,
		None,
		frost_properties().into(),
		None,
	))
}

/// Helper for session keys to map aura id
fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	_initial_authorities: Vec<(AuraId, GrandpaId)>,
	council_members: Vec<AccountId>,
	technical_committee_membership: Vec<AccountId>,
	root_key: AccountId,
	airdrop_creditor_account: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	let authorities = vec![
		(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			authority_keys_from_seed("Alice").0,
			authority_keys_from_seed("Alice").1,
		),
		(
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			authority_keys_from_seed("Bob").0,
			authority_keys_from_seed("Bob").1,
		),
	];
	let evm_genesis_account: AccountId =
		hex!["3f1ee662d59012a0001c2fca594083e4a104811646fa39111568448cb372a607"].into();

	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ICY * 40_000))
				.collect(),
		},
		aura: AuraConfig {
			authorities: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		sudo: SudoConfig {
			key: Some(root_key),
		},
		session: SessionConfig {
			keys: authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.1.clone(), x.2.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
					H160::from_str("8efcaf2c4ebbf88bf07f3bb44a2869c4c675ad7a")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address for benchmark usage
					H160::from_str("1000000000000000000000000000000000000001")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						nonce: U256::from(1),
						balance: U256::from(1_000_000_000_000_000_000_000_000u128),
						storage: Default::default(),
						code: vec![0x00],
					},
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		vesting: Default::default(),
		assets: AssetsConfig {
			assets: vec![
				// id, owner, is_sufficient, min_balance
				(1, evm_genesis_account.clone(), true, 1),
			],
			metadata: vec![
				// id, name, symbol, decimals
				(1, "Test Token".into(), "TICZ".into(), 10),
			],
			accounts: vec![
				// id, account_id, balance
				(1, evm_genesis_account.clone(), 100),
			],
		},
		council_membership: CouncilMembershipConfig {
			members: council_members.try_into().unwrap(),
			phantom: Default::default(),
		},
		council: CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		treasury: Default::default(),
		simple_inflation: Default::default(),
		fees_split: Default::default(),
		airdrop: AirdropConfig {
			creditor_account: airdrop_creditor_account,
			merkle_root: AIRDROP_MERKLE_ROOT,
		},
		technical_membership: TechnicalMembershipConfig {
			members: technical_committee_membership.try_into().unwrap(),
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		phragmen_election: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		democracy: DemocracyConfig::default(),
	}
}
