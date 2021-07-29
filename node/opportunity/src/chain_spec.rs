use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use sc_client_api::{BadBlocks, ForkBlocks};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use opportunity_runtime::{
	AccountId, AssetRegistryConfig, BabeConfig, BalancesConfig, Block,
	CouncilConfig, DemocracyConfig, ElectionsConfig, GrandpaConfig, ImOnlineConfig, OracleConfig,
	Perbill, SessionConfig, SessionKeys, Signature, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, TechnicalMembershipConfig, TokensConfig,
	TreasuryConfig
};

pub type AssetId = u32;
pub const CORE_ASSET_ID: AssetId = 1;

// Node `ChainSpec` extensions.
// Additional parameters for some Substrate core modules,
// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: BadBlocks<Block>,
}

//  The `ChainSpec` parameterized for the opportunity runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<opportunity_runtime::GenesisConfig, Extensions>;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const OPPORTUNITY_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 15,
            "tokenSymbol": "OPT"
        }"#;
const OPPORTUNITY_PROTOCOL_ID: &str = "opt";

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
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
pub fn opportunity_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../spec/opportunity_raw.json")[..])
}

pub fn opportunity_standalone_config() -> Result<ChainSpec, String> {
	let wasm_binary = opportunity_runtime::WASM_BINARY.ok_or("Development wasm not available")?;
	let boot_nodes = vec![];

	Ok(ChainSpec::from_genesis(
		// Name
		"Opportunity Standalone Testnet",
		// ID
		"opportunity_standalone",
		// Chain Type
		ChainType::Live,
		move || {
			opportunity_testnet_config_genesis(
				wasm_binary,
				// Initial authorities
				vec![authority_keys_from_seed("Alice")],
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
			)
		},
		// Bootnodes
		boot_nodes,
		// Telemetry
		Some(
			sc_telemetry::TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Telemetry url is valid"),
		),
		// Protocol ID
		Some(OPPORTUNITY_PROTOCOL_ID),
		// Properties
		serde_json::from_str(OPPORTUNITY_PROPERTIES).unwrap(),
		// Extensions
		Default::default(),
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = opportunity_runtime::WASM_BINARY.ok_or("Development wasm not available")?;
	let boot_nodes = vec![];

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		// Chain Type
		ChainType::Local,
		move || {
			opportunity_testnet_config_genesis(
				wasm_binary,
				// Initial authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
			)
		},
		// Bootnodes
		boot_nodes,
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		Default::default(),
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = opportunity_runtime::WASM_BINARY.ok_or("Development wasm not available")?;
	let boot_nodes = vec![];

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			opportunity_testnet_config_genesis(
				wasm_binary,
				// Initial authorities
				vec![authority_keys_from_seed("Alice")],
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
			)
		},
		// Bootnodes
		boot_nodes,
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		Default::default(),
	))
}

fn opportunity_testnet_config_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> opportunity_runtime::GenesisConfig {
	opportunity_runtime::GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		grandpa: GrandpaConfig::default(),
		sudo: SudoConfig { key: root_key },
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(opportunity_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						100_000_000_000_000_000_u128,
						StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
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
		oracle: OracleConfig {
			oracles: [get_account_id_from_seed::<sr25519::Public>("Alice")].to_vec(),
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((endowed_accounts.len() + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: TechnicalMembershipConfig::default(),
		treasury: TreasuryConfig::default(),
	}
}
