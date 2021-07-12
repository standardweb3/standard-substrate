use opportunity_runtime::{AccountId, Signature};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use opportunity_runtime::opaque::SessionKeys;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sc_service::ChainType;

use opportunity_runtime::{
	Perbill, TokensConfig, AssetRegistryConfig, OracleConfig, GrandpaConfig, AuraConfig, CouncilConfig, ElectionsConfig, TreasuryConfig, 
};

pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
pub type AssetId = u32;
pub const CORE_ASSET_ID: AssetId = 1;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<opportunity_runtime::GenesisConfig>;
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const OPPORTUNITY_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 15,
            "tokenSymbol": "OPT"
        }"#;
const OPPORTUNITY_PROTOCOL_ID: &str = "opt";

fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId,
) -> SessionKeys {
	SessionKeys {
		aura,
		grandpa,
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

pub fn authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	AuraId,
	GrandpaId,

) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<GrandpaId>(seed),

	)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Opportunity Testnet Chainspec.
/// Rust compiler is not deterministic. Therefore, compiled chainspec is shared to run the node for shared genesis block.
/// Reference: https://stackoverflow.com/questions/66554685/substrate-genesis-blocks-not-matching
pub fn opportunity_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../spec/opportunity_raw.json")[..]).unwrap()
}

pub fn opportunity_standalone_config() -> ChainSpec {
    ChainSpec::from_genesis(
        // Name
        "Opportunity Standalone Testnet",
        // ID
        "opportunity_standalone",
        ChainType::Live,
        move || {
            testnet_genesis(
                vec![authority_keys_from_seed("Alice")],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
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
								vec![
									get_from_seed::<AuraId>("Alice"),
									get_from_seed::<AuraId>("Bob"),
									get_from_seed::<AuraId>("Alice//stash"),
									get_from_seed::<AuraId>("Bob//stash"),
								],
            )
        },
        vec![],
        Some(
            sc_telemetry::TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Telemetry url is valid"),
        ),
        Some(OPPORTUNITY_PROTOCOL_ID),
        serde_json::from_str(OPPORTUNITY_PROPERTIES).unwrap(),
        Default::default(),
    )
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				vec![authority_keys_from_seed("Alice")],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
					get_from_seed::<AuraId>("Alice//stash"),
					get_from_seed::<AuraId>("Bob//stash"),
				],
			)
		},
		vec![],
		None,
		None,
		None,
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				vec![authority_keys_from_seed("Alice")],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
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
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
					get_from_seed::<AuraId>("Alice//stash"),
					get_from_seed::<AuraId>("Bob//stash"),
				],
			)
		},
		vec![],
		None,
		None,
		None,
		None
	)
}

fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		AuraId,
		GrandpaId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	initials: Vec<AuraId>,
) -> opportunity_runtime::GenesisConfig {
	opportunity_runtime::GenesisConfig {
		system: opportunity_runtime::SystemConfig {
			code: opportunity_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: opportunity_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.3.clone(), 1)).collect(),
		},
		sudo: opportunity_runtime::SudoConfig { key: root_key },
		tokens: TokensConfig {
			endowed_accounts: endowed_accounts.iter().flat_map(|_x| vec![]).collect(),
		},
		asset_registry: AssetRegistryConfig {
			core_asset_id: CORE_ASSET_ID,
			asset_ids: vec![
				(b"STD".to_vec(), 1),
				(b"MTR".to_vec(), 2),
				(b"DOT".to_vec(), 3),
				(b"KSM".to_vec(), 4),
			],
			next_asset_id: 5,
		},
		oracle: OracleConfig{
			oracles: [get_account_id_from_seed::<sr25519::Public>("Alice")].to_vec()
		},
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		treasury: TreasuryConfig::default()
	}
}