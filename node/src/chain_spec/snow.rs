use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use snow_runtime::currency::ICY;
use snow_runtime::{
	wasm_binary_unwrap, AccountId, AirdropConfig, AuraConfig, AuraId, Balance, BalancesConfig,
	CollatorSelectionConfig, CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig,
	GenesisConfig, IndicesConfig, ParachainInfoConfig, PolkadotXcmConfig, SS58Prefix,
	SessionConfig, SessionKeys, Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	TechnicalMembershipConfig, TreasuryPalletId, VestingConfig,
};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{AccountIdConversion, IdentifyAccount, Verify};
use sp_runtime::BoundedVec;
use std::collections::BTreeMap;

use super::{get_from_seed, Extensions};

/// Publicly expose SnowChainSpec for sc service
pub type SnowChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const PARA_ID: u32 = 2000;
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

//const TOTAL_SUPPLY: Balance = ICY * 1800000000;
//const TOTAL_AIR_DROP: Balance = 1 * ICY;

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex!("b654eac2f99abbe8e847a2079a2018bcf09989c00a3e0dd0114a335c4d97ef32");

fn snow_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("tokenSymbol".into(), "ICZ".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	properties
}

/// Gen Snow chain specification.
pub fn get_dev_chain_spec() -> SnowChainSpec {
	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	let invulnerables = vec![
		get_authority_keys_from_seed("Alice"),
		get_authority_keys_from_seed("Bob"),
	];

	let authorities = vec![
		(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_from_seed::<AuraId>("Alice"),
		),
		(
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_from_seed::<AuraId>("Bob"),
		),
	];

	let endowed_accounts = vec![
		((
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			ICY * 300_000_000,
		)),
		((
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			ICY * 300_000_000,
		)),
	];

	let council_members = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let technical_committee = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	SnowChainSpec::from_genesis(
		"Snow Development",
		"snow-dev",
		ChainType::Development,
		move || {
			make_genesis(
				root_key.clone(),
				authorities.clone(),
				invulnerables.clone(),
				endowed_accounts.clone(),
				council_members.clone(),
				technical_committee.clone(),
				airdrop_creditor_account.clone(),
				PARA_ID.into(),
			)
		},
		vec![],
		None,
		"snow".into(),
		None,
		Some(snow_properties()),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
		},
	)
}

pub fn testnet_spec() -> SnowChainSpec {
	let root_key: AccountId =
		hex!["6f38cb15a6ec17a68f2aec60d2cd8cd15e58b4e33ee7f705d1cbcde07009d33f"].into();

	let invulnerables = vec![(
		hex!["f28ae952b7518dbc35543b894facca7db5ab982ec6aa9afbba4e8c015ce4b74a"].into(),
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].unchecked_into(),
	)];
	let authorities = vec![(
		hex!["f28ae952b7518dbc35543b894facca7db5ab982ec6aa9afbba4e8c015ce4b74a"].into(),
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].unchecked_into(),
	)];

	let airdrop_creditor_account: AccountId =
		hex!["1d94604ab70381bc1ea9e14854b939f4c651d2259aa0f7eb193971d526f64a45"].into();

	let endowed_accounts = vec![
		(
			hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into(),
			ICY * 931000000,
		),
		(
			TreasuryPalletId::get().into_account_truncating(),
			ICY * 1729000000,
		),
		(
			hex!["6f38cb15a6ec17a68f2aec60d2cd8cd15e58b4e33ee7f705d1cbcde07009d33f"].into(),
			ICY * 2000,
		),
		(
			hex!["f28ae952b7518dbc35543b894facca7db5ab982ec6aa9afbba4e8c015ce4b74a"].into(),
			ICY * 5100,
		),
		(
			hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
			ICY * 2000,
		),
		(
			hex!["328d54003810edf7cef62d1374032333ade2fdb2756138fc43f6b4c1918bef7c"].into(),
			ICY * 2000,
		),
		(
			hex!["f057f9fbec27bb5b92c5f30e89cae9826f5b86cae8380aa383c079939b3e0a55"].into(),
			ICY * 2000,
		),
		(
			hex!["6adaa753d9c17d9280d2469acdac1aa9b7f01be3d4149f667b9be7c7fbad1319"].into(),
			ICY * 2000,
		),
	];

	let council_members = vec![];

	let technical_committee = vec![
		hex!["f28ae952b7518dbc35543b894facca7db5ab982ec6aa9afbba4e8c015ce4b74a"].into(),
		hex!["f057f9fbec27bb5b92c5f30e89cae9826f5b86cae8380aa383c079939b3e0a55"].into(),
		hex!["70d8131ab823528226296bbfbb5827a5ae84beda0edf73f0cbc95057ef43be6a"].into(),
	];

	SnowChainSpec::from_genesis(
		"Snow Local Tesnet",
		"snow-testnet",
		ChainType::Local,
		move || {
			make_genesis(
				root_key.clone(),
				authorities.clone(),
				invulnerables.clone(),
				endowed_accounts.clone(),
				council_members.clone(),
				technical_committee.clone(),
				airdrop_creditor_account.clone(),
				PARA_ID.into(),
			)
		},
		vec![],
		None,
		"snow".into(),
		None,
		Some(snow_properties()),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
		},
	)
}

/// Helper function to create GenesisConfig.
fn make_genesis(
	root_key: AccountId,
	authorities: Vec<(AccountId, AuraId)>,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	council_members: Vec<AccountId>,
	technical_committee: Vec<AccountId>,
	airdrop_creditor_account: AccountId,
	parachain_id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
		},
		sudo: SudoConfig {
			key: Some(root_key),
		},
		parachain_info: ParachainInfoConfig { parachain_id },
		balances: BalancesConfig {
			balances: endowed_accounts,
		},
		vesting: VestingConfig { vesting: vec![] },
		aura: AuraConfig {
			authorities: vec![],
		},
		aura_ext: Default::default(),
		collator_selection: CollatorSelectionConfig {
			desired_candidates: 200,
			candidacy_bond: 5000 * ICY,
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
		},
		session: SessionConfig {
			keys: authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone())))
				.collect::<Vec<_>>(),
		},
		evm: EVMConfig {
			accounts: { BTreeMap::new() },
		},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		assets: Default::default(),
		council_membership: CouncilMembershipConfig {
			members: BoundedVec::truncate_from(council_members),
			phantom: Default::default(),
		},
		council: CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		ethereum: Default::default(),
		treasury: Default::default(),
		parachain_system: Default::default(),
		simple_inflation: Default::default(),
		fees_split: Default::default(),
		airdrop: AirdropConfig {
			creditor_account: airdrop_creditor_account,
			merkle_root: AIRDROP_MERKLE_ROOT,
		},
		technical_membership: TechnicalMembershipConfig {
			members: BoundedVec::truncate_from(technical_committee),
			phantom: Default::default(),
		},
		phragmen_election: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		democracy: DemocracyConfig::default(),
		polkadot_xcm: PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		}, // Default::default(),
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AuraId) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
	)
}

/// Helper for session keys to map aura id
fn session_keys(aura: AuraId) -> SessionKeys {
	SessionKeys { aura }
}

pub fn snow_kusama_config() -> Result<SnowChainSpec, String> {
	sc_chain_spec::GenericChainSpec::from_json_bytes(
		&include_bytes!("../../../resources/snow-kusama.json")[..],
	)
}

pub fn snow_staging_rococo_config() -> Result<SnowChainSpec, String> {
	sc_chain_spec::GenericChainSpec::from_json_bytes(
		&include_bytes!("../../../resources/snow-staging-rococo.json")[..],
	)
}
