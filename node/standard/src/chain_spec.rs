use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use standard_runtime::{
	AccountId, AssetRegistryConfig, AuraConfig, AuraId, BalancesConfig, GenesisConfig,
	ImOnlineConfig, ImOnlineId, OracleConfig, ParachainInfoConfig, SessionConfig, SessionKeys,
	Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig, VestingConfig, WASM_BINARY,
};

use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use primitives::AssetId;

pub const CORE_ASSET_ID: AssetId = 1;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const STANDARD_PROPERTIES: &str = r#"
        {
            "ss58Format": 42,
            "tokenDecimals": 15,
            "tokenSymbol": "STND"
        }"#;
const STANDARD_PROTOCOL_ID: &str = "standard";

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

pub fn standard_rococo_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../spec/standard_rococo_raw.json")[..])
}

pub fn standard_barocco_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../spec/standard_barocco_raw.json")[..])
}

pub fn standard_kusama_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../spec/standard_kusama_raw.json")[..])
}

pub fn standard_kusama_genesis_config(id: ParaId) -> Result<ChainSpec, String> {
	use hex_literal::hex;

	Ok(ChainSpec::from_genesis(
		// Name
		"Standard Kusama Parachain",
		// ID
		"standard_ksm_parachain",
		// Chain Type
		ChainType::Live,
		move || {
			testnet_genesis(
				// Sudo account
				// 5EUxKM69tZmKDyocwmdiDJdtmgipEXVkfytMbiCAH1P6Q9W9
				hex!["6af70880fe4b040979009fd07dfbe631c355088c285a27061e883e0c0fbbe907"].into(),
				// Initial authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Pre-funded accounts
				vec![
					hex!["6af70880fe4b040979009fd07dfbe631c355088c285a27061e883e0c0fbbe907"].into(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		Some(
			sc_telemetry::TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Telemetry url is valid"),
		),
		// Protocol ID
		Some(STANDARD_PROTOCOL_ID),
		// Properties
		serde_json::from_str(STANDARD_PROPERTIES).unwrap(),
		// Extensions
		Extensions { relay_chain: "kusama".into(), para_id: id.into() },
	))
}

pub fn standard_rococo_genesis_config(id: ParaId) -> Result<ChainSpec, String> {
	use hex_literal::hex;

	Ok(ChainSpec::from_genesis(
		// Name
		"Standard Rococo Parachain",
		// ID
		"standard_rococo_parachain",
		// Chain Type
		ChainType::Live,
		move || {
			testnet_genesis(
				// Sudo account
				// ZHd7drSUrpJfkkYYjMoKfCwtyN5SU6qSiQrA4BoESiuCTTa
				hex!["9434f808bdb12725c67d7dca1f2584970c0c702215508fbd148e0262f2a15e00"].into(),
				// Initial authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Pre-funded accounts
				vec![
					hex!["9434f808bdb12725c67d7dca1f2584970c0c702215508fbd148e0262f2a15e00"].into(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		Some(
			sc_telemetry::TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Telemetry url is valid"),
		),
		// Protocol ID
		Some(STANDARD_PROTOCOL_ID),
		// Properties
		serde_json::from_str(STANDARD_PROPERTIES).unwrap(),
		// Extensions
		Extensions { relay_chain: "rococo".into(), para_id: id.into() },
	))
}

pub fn development_config(id: ParaId) -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
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
		Extensions { relay_chain: "rococo-dev".into(), para_id: id.into() },
	))
}

pub fn local_config(id: ParaId) -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial authorities
				vec![authority_keys_from_seed("Alice")],
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
				// Parachain ID
				id,
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
		Extensions { relay_chain: "rococo-local".into(), para_id: id.into() },
	))
}

fn session_keys(aura: AuraId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, im_online }
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, AccountId, AuraId, ImOnlineId)>,
	endowed_accounts: Vec<AccountId>,
	parachain_id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("WASM binary was not build, please build it!").to_vec(),
			changes_trie_config: Default::default(),
		},
		sudo: SudoConfig { key: root_key },
		parachain_system: Default::default(),
		parachain_info: ParachainInfoConfig { parachain_id },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		vesting: VestingConfig { vesting: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone())))
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
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		aura: AuraConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		aura_ext: Default::default(),
		asset_registry: AssetRegistryConfig {
			core_asset_id: CORE_ASSET_ID,
			asset_ids: vec![
				(b"STND".to_vec(), 1),
				(b"MTR".to_vec(), 2),
				(b"DOT".to_vec(), 3),
				(b"KSM".to_vec(), 4),
				(b"ROC".to_vec(), 5),
			],
			next_asset_id: 6,
		},
		oracle: OracleConfig {
			oracles: [get_account_id_from_seed::<sr25519::Public>("Alice")].to_vec(),
			provider_count: 5,
		},
	}
}
