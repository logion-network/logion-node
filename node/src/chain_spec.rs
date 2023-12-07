use logion_node_runtime::{
	AccountId, AuraConfig, BalancesConfig, RuntimeGenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
	opaque::SessionKeys,
	ValidatorSetConfig, SessionConfig, LoAuthorityListConfig, Balance,
};
use pallet_lo_authority_list::GenesisHostData;
use sc_service::ChainType;
use serde_json::json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ed25519, sr25519, Pair, Public, OpaquePeerId};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
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
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s)
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"logion Development",
		// ID
		"logion_dev",
		ChainType::Development,
		move || logion_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
			],
			vec![ // Initial set of Logion Legal Officers
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					GenesisHostData {
						node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
						base_url: None,
						region: "Europe".into(),
					},
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					GenesisHostData {
						node_id: Some(OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap())),
						base_url: None,
						region: "Europe".into(),
					}
				),
			],
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(default_properties()),
		// Extensions
		None,
	))
}

pub fn mvp_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	const ROOT_PUBLIC_SR25519: &str = "5FUg3QWfipPf8yKv5hMK6wQf8nn6og9BbRNcr3Y8CwUJwTh9";

	const NODE1_PUBLIC_SR25519: &str = "5DjzFDhFidvGCuuy6i8Lsi4XyruYjxTTkJKb1o7XzVdMNPVb";
	const NODE1_PUBLIC_ED25519: &str = "5EVSLLEFUhrWtb5n7tC7ud91nT1qFodhYkAkxdbNpJznqTZ5";

	const NODE2_PUBLIC_SR25519: &str = "5DoD9n61SssFiWQDTD7bz1eX3KCxZJ6trVj2GsDwMi2PqP85";
	const NODE2_PUBLIC_ED25519: &str = "5CUJgAjKLb64bHFFbLu5hQzgR28zH6apcymSDLV1RBFujVjW";

	const NODE3_PUBLIC_SR25519: &str = "5CJTSSJ4v1RAauZpeqTeddyui4wESZZqPor33wum9aKuQXZC";
	const NODE3_PUBLIC_ED25519: &str = "5FuUhqoi1BhAf92K5DnKPUFDrYNDX4JUAQKgT3AvCNewjpTw";

	Ok(ChainSpec::from_genesis(
		// Name
		"logion MVP",
		// ID
		"logion_mvp",
		ChainType::Live,
		move || logion_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				(
					AccountId::from_str(NODE1_PUBLIC_SR25519).unwrap(),
					AuraId::from(sr25519::Public::from_str(NODE1_PUBLIC_SR25519).unwrap()),
					GrandpaId::from(ed25519::Public::from_str(NODE1_PUBLIC_ED25519).unwrap()),
				),
				(
					AccountId::from_str(NODE2_PUBLIC_SR25519).unwrap(),
					AuraId::from(sr25519::Public::from_str(NODE2_PUBLIC_SR25519).unwrap()),
					GrandpaId::from(ed25519::Public::from_str(NODE2_PUBLIC_ED25519).unwrap()),
				)
				,
				(
					AccountId::from_str(NODE3_PUBLIC_SR25519).unwrap(),
					AuraId::from(sr25519::Public::from_str(NODE3_PUBLIC_SR25519).unwrap()),
					GrandpaId::from(ed25519::Public::from_str(NODE3_PUBLIC_ED25519).unwrap()),
				)
			],
			// Sudo account
			AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
			// Pre-funded accounts
			vec![
				AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
				AccountId::from_str(NODE1_PUBLIC_SR25519).unwrap(),
				AccountId::from_str(NODE2_PUBLIC_SR25519).unwrap(),
				AccountId::from_str(NODE3_PUBLIC_SR25519).unwrap(),
			],
			vec![ // Initial set of Logion Legal Officers
			],
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(default_properties()),
		// Extensions
		None,
	))
}

pub fn test_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"logion Testnet",
		// ID
		"logion_test_testnet",
		ChainType::Live,
		move || logion_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
				authority_keys_from_seed("Charlie"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
			],
			vec![ // Initial set of Logion Legal Officers
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					GenesisHostData {
						node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
						base_url: None,
						region: "Europe".into(),
					}
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					GenesisHostData {
						node_id: Some(OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap())),
						base_url: None,
						region: "Europe".into(),
					}
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					GenesisHostData {
						node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ").into_vec().unwrap())),
						base_url: None,
						region: "Europe".into(),
					}
				),
			],
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(default_properties()),
		// Extensions
		None,
	))
}

const INITIAL_BALANCE: Balance = 100_000_000_000_000_000_000_000;

/// Configure initial storage state for pallets.
fn logion_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	legal_officers: Vec<(AccountId, GenesisHostData)>,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance.
			balances: endowed_accounts.iter().cloned().map(|k|(k, INITIAL_BALANCE)).collect(),
		},
		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone())))
				.collect::<Vec<_>>(),
		},
		aura: AuraConfig {
			authorities: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		lo_authority_list: LoAuthorityListConfig {
			legal_officers,
		},
		transaction_payment: Default::default(),
		logion_treasury: Default::default(),
		community_treasury: Default::default(),
	}
}

fn default_properties() -> sc_service::Properties {
	let mut props : sc_service::Properties = sc_service::Properties::new();
	props.insert("tokenSymbol".to_string(), json!("LGNT"));
	props.insert("tokenDecimals".to_string(), json!(18));
	return props;
}
