use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use snow_runtime::currency::ICY;
use snow_runtime::{
	wasm_binary_unwrap, AccountId, AirdropConfig, AuraConfig, AuraId, BalancesConfig,
	CollatorSelectionConfig, CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig,
	GenesisConfig, IndicesConfig, ParachainInfoConfig, SS58Prefix, SessionConfig, SessionKeys,
	Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig, TechnicalMembershipConfig,
	VestingConfig,
};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;

use super::{get_from_seed, Extensions};

/// Publicly expose SnowChainSpec for sc service
pub type SnowChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex!("990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122");

const PARA_ID: u32 = 2000;

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
		(get_account_id_from_seed::<sr25519::Public>("Alice")),
		(get_account_id_from_seed::<sr25519::Public>("Bob")),
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
		None,
		None,
		Some(snow_properties()),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
		},
	)
}

/// Helper function to create Arctic GenesisConfig.
fn make_genesis(
	root_key: AccountId,
	authorities: Vec<(AccountId, AuraId)>,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
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
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ICY * 300_000_000))
				.collect(),
		},
		vesting: VestingConfig { vesting: vec![] },
		aura: AuraConfig {
			authorities: vec![],
		},
		aura_ext: Default::default(),
		collator_selection: CollatorSelectionConfig {
			desired_candidates: 200,
			candidacy_bond: 32_000 * ICY,
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
			members: council_members,
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
			members: technical_committee,
			phantom: Default::default(),
		},
		phragmen_election: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		democracy: DemocracyConfig::default(),
		polkadot_xcm: Default::default(),
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
