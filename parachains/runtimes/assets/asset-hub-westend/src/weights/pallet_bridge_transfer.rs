// Copyright Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_bridge_transfer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westmint-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_bridge_transfer
// --chain=westmint-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/westmint/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_transfer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_transfer::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BridgeTransfer AllowedExporters (r:1 w:0)
	/// Proof: BridgeTransfer AllowedExporters (max_values: None, max_size: Some(2537), added: 5012, mode: MaxEncodedLen)
	/// Storage: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Proof Skipped: unknown `0x3a696e747261626c6f636b5f656e74726f7079` (r:1 w:1)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:2 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(134), added: 2609, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem RelevantMessagingState (r:1 w:0)
	/// Proof Skipped: ParachainSystem RelevantMessagingState (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmpQueue OutboundXcmpStatus (r:1 w:1)
	/// Proof Skipped: XcmpQueue OutboundXcmpStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmpQueue OutboundXcmpMessages (r:0 w:1)
	/// Proof Skipped: XcmpQueue OutboundXcmpMessages (max_values: None, max_size: None, mode: Measured)
	fn transfer_asset_via_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `732`
		//  Estimated: `6208`
		// Minimum execution time: 151_961_000 picoseconds.
		Weight::from_parts(153_658_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
