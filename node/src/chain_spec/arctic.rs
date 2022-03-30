use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use arctic_runtime::{
    wasm_binary_unwrap, AccountId, AuraConfig, AuraId, Balance, BalancesConfig,
    CollatorSelectionConfig, EVMConfig, GenesisConfig, ParachainInfoConfig, CouncilConfig,
    SessionConfig, Signature, SudoConfig, SystemConfig, VestingConfig, SessionKeys
};
use arctic_runtime::currency::{ICY};
use sp_core::{sr25519, Pair, Public};
use std::{collections::BTreeMap};

use sp_runtime::traits::{IdentifyAccount, Verify};
use std::marker::PhantomData;
use super::{get_from_seed, Extensions};


/// Publicly expose ArcticChainSpec for sc service
pub type ArcticChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const ARCTIC_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 18,
            "tokenSymbol": "ICZ"
        }"#;


/// Gen Arctic chain specification for given parachain id.
pub fn get_chain_spec(para_id: u32) -> ArcticChainSpec {
    // Alice as default
    let sudo_key = get_account_id_from_seed::<sr25519::Public>("Alice");
    let endowned = vec![
        (
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            1 << 70,
        ),
        (get_account_id_from_seed::<sr25519::Public>("Bob"), 1 << 70),
    ];

    ArcticChainSpec::from_genesis(
        "Arctic Testnet",
        "arctic",
        ChainType::Development,
        move || make_genesis(endowned.clone(), 
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice")
        ],
        sudo_key.clone(), 
        para_id.into()),
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
    balances: Vec<(AccountId, Balance)>,
    council_members: Vec<AccountId>,
    root_key: AccountId,
    parachain_id: ParaId,
) -> GenesisConfig {
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

    // This is supposed the be the simplest bytecode to revert without returning any data.
    // We will pre-deploy it under all of our precompiles to ensure they can be called from
    // within contracts.
    // (PUSH1 0x00 PUSH1 0x00 REVERT)

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
        },
        sudo: SudoConfig {
            key: Some(root_key),
        },
        parachain_info: ParachainInfoConfig { parachain_id },
        balances: BalancesConfig { balances },
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
			accounts: {
				let map = BTreeMap::new();
				map
			},
		},
        dynamic_fee: Default::default(),
        base_fee: Default::default(),
        assets: Default::default(),
        council: CouncilConfig {
            members: council_members,
            phantom: PhantomData,
        },
        ethereum: Default::default(),
        treasury: Default::default(),
        polkadot_xcm: Default::default(),
        parachain_system: Default::default()
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
