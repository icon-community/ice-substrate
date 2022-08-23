use super::{get_from_seed, Extensions};
use arctic_runtime::currency::ICY;
use arctic_runtime::{
	wasm_binary_unwrap, AccountId, AirdropConfig, AuraConfig, AuraId, BalancesConfig,
	CollatorSelectionConfig, CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig,
	GenesisConfig, IndicesConfig, ParachainInfoConfig, PolkadotXcmConfig, SS58Prefix,
	SessionConfig, SessionKeys, Signature, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	TechnicalMembershipConfig, VestingConfig,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;

/// Publicly expose ArcticChainSpec for sc service
pub type ArcticChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex!("b654eac2f99abbe8e847a2079a2018bcf09989c00a3e0dd0114a335c4d97ef32");

const PARA_ID: u32 = 2000;
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

fn arctic_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("tokenSymbol".into(), "ICZ".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	properties
}

/// Gen Arctic chain specification.
pub fn get_chain_spec() -> ArcticChainSpec {
	let endowed_accounts = vec![
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
		hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].into(),
		hex!["98003761bff94c8c44af38b8a92c1d5992d061d41f700c76255c810d447d613f"].into(),
	];

	let authorities = vec![
		(
			hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
			hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"]
				.unchecked_into(),
		),
		(
			hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].into(),
			hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"]
				.unchecked_into(),
		),
	];

	let council_members =
		vec![hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into()];

	let technical_committee =
		vec![hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into()];

	let root_key: AccountId =
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into();

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	ArcticChainSpec::from_genesis(
		"Arctic",
		"arctic",
		ChainType::Live,
		move || {
			make_genesis(
				endowed_accounts.clone(),
				authorities.clone(),
				council_members.clone(),
				technical_committee.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				PARA_ID.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(arctic_properties()),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "rococo".into(),
			para_id: PARA_ID,
		},
	)
}

/// Gen Arctic chain specification.
pub fn get_dev_chain_spec() -> ArcticChainSpec {
	let endowed_accounts = vec![
		(get_account_id_from_seed::<sr25519::Public>("Alice")),
		(get_account_id_from_seed::<sr25519::Public>("Bob")),
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

	let council_members = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let technical_committee = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	ArcticChainSpec::from_genesis(
		"Arctic Dev",
		"arctic-dev",
		ChainType::Development,
		move || {
			make_genesis(
				endowed_accounts.clone(),
				authorities.clone(),
				council_members.clone(),
				technical_committee.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				PARA_ID.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(arctic_properties()),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
		},
	)
}

/// Helper for session keys to map aura id
fn session_keys(aura: AuraId) -> SessionKeys {
	SessionKeys { aura }
}

/// Helper function to create Arctic GenesisConfig.
fn make_genesis(
	endowed_accounts: Vec<AccountId>,
	authorities: Vec<(AccountId, AuraId)>,
	council_members: Vec<AccountId>,
	technical_committee: Vec<AccountId>,
	root_key: AccountId,
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
			invulnerables: authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
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
		polkadot_xcm: PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
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
