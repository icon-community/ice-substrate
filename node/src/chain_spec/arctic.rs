use super::{get_from_seed, Extensions};
use arctic_runtime::currency::ICY;
use arctic_runtime::{
	wasm_binary_unwrap, AccountId, AuraConfig, AuraId, BalancesConfig, CollatorSelectionConfig,
	CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig, GenesisConfig,
	IndicesConfig, ParachainInfoConfig, SessionConfig, SessionKeys, Signature, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, TechnicalMembershipConfig, VestingConfig, AirdropConfig
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;

/// Publicly expose ArcticChainSpec for sc service
pub type ArcticChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const ARCTIC_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 18,
            "tokenSymbol": "ICZ"
        }"#;

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex_literal::hex!("990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122");

/// Gen Arctic chain specification for given parachain id.
pub fn get_chain_spec(para_id: u32) -> ArcticChainSpec {
	ArcticChainSpec::from_genesis(
		"Arctic Testnet",
		"arctic",
		ChainType::Live,
		move || {
			make_genesis(
				// Endowed accounts
				vec![
					hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
					hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].into(),
					hex!["98003761bff94c8c44af38b8a92c1d5992d061d41f700c76255c810d447d613f"].into(),
				],
				// Initial PoA authorities
				vec![
					(
						hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"]
							.into(),
						hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"]
							.unchecked_into(),
					),
					(
						hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"]
							.into(),
						hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"]
							.unchecked_into(),
					),
				],
				// Council members
				vec![
					hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
				],
				vec![
					hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
				],
				// Sudo account
				hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
				// Airdrop creditor account
				hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into(),
				para_id.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		serde_json::from_str(ARCTIC_PROPERTIES).unwrap(),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "arctic".into(),
			para_id,
		},
	)
}

/// Gen Arctic chain specification for given parachain id.
pub fn get_dev_chain_spec(para_id: u32) -> ArcticChainSpec {
	// Alice as default
	let sudo_key = get_account_id_from_seed::<sr25519::Public>("Alice");
	let endowned = vec![
		(get_account_id_from_seed::<sr25519::Public>("Alice")),
		(get_account_id_from_seed::<sr25519::Public>("Bob")),
	];

	ArcticChainSpec::from_genesis(
		"Arctic Dev",
		"arctic-dev",
		ChainType::Development,
		move || {
			make_genesis(
				// Endowed accounts
				endowned.clone(),
				// Initial PoA authorities
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_from_seed::<AuraId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_from_seed::<AuraId>("Bob"),
					),
				],
				// Council members
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				// Sudo account
				sudo_key.clone(),
				// Airdrop creditor account
				hex!("10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872").into(),
				para_id.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		serde_json::from_str(ARCTIC_PROPERTIES).unwrap(),
		Extensions {
			bad_blocks: Default::default(),
			relay_chain: "arctic".into(),
			para_id,
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
	airdrop_creditor_account: [u8; 32],
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
		polkadot_xcm: Default::default(),
		parachain_system: Default::default(),
		simple_inflation: Default::default(),
		fees_split: Default::default(),
		airdrop: AirdropConfig {
			creditor_account: sp_runtime::AccountId32::new(airdrop_creditor_account),
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
