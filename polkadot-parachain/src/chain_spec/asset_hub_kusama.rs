// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, Extensions, SAFE_XCM_VERSION,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance as AssetHubBalance};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type AssetHubKusamaChainSpec =
	sc_service::GenericChainSpec<asset_hub_kusama_runtime::RuntimeGenesisConfig, Extensions>;

const ASSET_HUB_KUSAMA_ED: AssetHubBalance =
	asset_hub_kusama_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn asset_hub_kusama_session_keys(keys: AuraId) -> asset_hub_kusama_runtime::SessionKeys {
	asset_hub_kusama_runtime::SessionKeys { aura: keys }
}

pub fn asset_hub_kusama_development_config() -> AssetHubKusamaChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	AssetHubKusamaChainSpec::from_genesis(
		// Name
		"Kusama Asset Hub Development",
		// ID
		"asset-hub-kusama-dev",
		ChainType::Local,
		move || {
			asset_hub_kusama_genesis(
				// initial collators.
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed::<AuraId>("Alice"),
				)],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama-dev".into(), para_id: 1000 },
	)
}

pub fn asset_hub_kusama_local_config() -> AssetHubKusamaChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	AssetHubKusamaChainSpec::from_genesis(
		// Name
		"Kusama Asset Hub Local",
		// ID
		"asset-hub-kusama-local",
		ChainType::Local,
		move || {
			asset_hub_kusama_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed::<AuraId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed::<AuraId>("Bob"),
					),
				],
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
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama-local".into(), para_id: 1000 },
	)
}

pub fn asset_hub_kusama_config() -> AssetHubKusamaChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 2.into());
	properties.insert("tokenSymbol".into(), "KSM".into());
	properties.insert("tokenDecimals".into(), 12.into());

	AssetHubKusamaChainSpec::from_genesis(
		// Name
		"Kusama Asset Hub",
		// ID
		"asset-hub-kusama",
		ChainType::Live,
		move || {
			asset_hub_kusama_genesis(
				// initial collators.
				vec![
					(
						hex!("50673d59020488a4ffc9d8c6de3062a65977046e6990915617f85fef6d349730")
							.into(),
						hex!("50673d59020488a4ffc9d8c6de3062a65977046e6990915617f85fef6d349730")
							.unchecked_into(),
					),
					(
						hex!("fe8102dbc244e7ea2babd9f53236d67403b046154370da5c3ea99def0bd0747a")
							.into(),
						hex!("fe8102dbc244e7ea2babd9f53236d67403b046154370da5c3ea99def0bd0747a")
							.unchecked_into(),
					),
					(
						hex!("38144b5398e5d0da5ec936a3af23f5a96e782f676ab19d45f29075ee92eca76a")
							.into(),
						hex!("38144b5398e5d0da5ec936a3af23f5a96e782f676ab19d45f29075ee92eca76a")
							.unchecked_into(),
					),
					(
						hex!("3253947640e309120ae70fa458dcacb915e2ddd78f930f52bd3679ec63fc4415")
							.into(),
						hex!("3253947640e309120ae70fa458dcacb915e2ddd78f930f52bd3679ec63fc4415")
							.unchecked_into(),
					),
				],
				Vec::new(),
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "kusama".into(), para_id: 1000 },
	)
}

fn asset_hub_kusama_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> asset_hub_kusama_runtime::RuntimeGenesisConfig {
	asset_hub_kusama_runtime::RuntimeGenesisConfig {
		system: asset_hub_kusama_runtime::SystemConfig {
			code: asset_hub_kusama_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: asset_hub_kusama_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ASSET_HUB_KUSAMA_ED * 524_288))
				.collect(),
		},
		parachain_info: asset_hub_kusama_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: asset_hub_kusama_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: ASSET_HUB_KUSAMA_ED * 16,
			..Default::default()
		},
		session: asset_hub_kusama_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                         // account id
						acc,                                 // validator id
						asset_hub_kusama_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: asset_hub_kusama_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}