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

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-polkadot-dev"), DB CACHE: 1024

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
// --pallet=pallet_xcm_benchmarks::generic
// --chain=bridge-hub-polkadot-dev
// --header=./file_header.txt
// --template=./templates/xcm-bench-template.hbs
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-polkadot/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 30_819_000 picoseconds.
		Weight::from_parts(31_157_000, 3535)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_869_000 picoseconds.
		Weight::from_parts(2_920_000, 0)
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	pub fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `3497`
		// Minimum execution time: 10_268_000 picoseconds.
		Weight::from_parts(10_496_000, 3497)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_990_000 picoseconds.
		Weight::from_parts(12_206_000, 0)
	}
	pub fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_170_000 picoseconds.
		Weight::from_parts(3_308_000, 0)
	}
	pub fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_650_000 picoseconds.
		Weight::from_parts(2_783_000, 0)
	}
	pub fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_681_000 picoseconds.
		Weight::from_parts(2_829_000, 0)
	}
	pub fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_622_000 picoseconds.
		Weight::from_parts(2_688_000, 0)
	}
	pub fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_385_000 picoseconds.
		Weight::from_parts(3_538_000, 0)
	}
	pub fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_630_000 picoseconds.
		Weight::from_parts(2_720_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 24_446_000 picoseconds.
		Weight::from_parts(24_854_000, 3535)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	// Proof Skipped: PolkadotXcm AssetTraps (max_values: None, max_size: None, mode: Measured)
	pub fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `90`
		//  Estimated: `3555`
		// Minimum execution time: 14_713_000 picoseconds.
		Weight::from_parts(15_010_000, 3555)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_702_000 picoseconds.
		Weight::from_parts(2_744_000, 0)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38`
		//  Estimated: `3503`
		// Minimum execution time: 25_955_000 picoseconds.
		Weight::from_parts(26_632_000, 3503)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	pub fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_965_000 picoseconds.
		Weight::from_parts(5_168_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 27_707_000 picoseconds.
		Weight::from_parts(28_081_000, 3535)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_215_000 picoseconds.
		Weight::from_parts(4_362_000, 0)
	}
	pub fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_843_000 picoseconds.
		Weight::from_parts(2_957_000, 0)
	}
	pub fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_751_000 picoseconds.
		Weight::from_parts(2_809_000, 0)
	}
	pub fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_674_000 picoseconds.
		Weight::from_parts(2_737_000, 0)
	}
	pub fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_891_000 picoseconds.
		Weight::from_parts(2_952_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 28_600_000 picoseconds.
		Weight::from_parts(29_001_000, 3535)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_748_000 picoseconds.
		Weight::from_parts(4_813_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `3535`
		// Minimum execution time: 25_483_000 picoseconds.
		Weight::from_parts(25_737_000, 3535)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_755_000 picoseconds.
		Weight::from_parts(2_817_000, 0)
	}
	pub fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_700_000 picoseconds.
		Weight::from_parts(2_773_000, 0)
	}
	pub fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_670_000 picoseconds.
		Weight::from_parts(2_711_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: BridgeKusamaMessages PalletOperatingMode (r:1 w:0)
	// Proof: BridgeKusamaMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	// Storage: BridgeKusamaMessages OutboundLanes (r:1 w:1)
	// Proof: BridgeKusamaMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added: 539, mode: MaxEncodedLen)
	// Storage: BridgeKusamaMessages OutboundMessages (r:0 w:1)
	// Proof: BridgeKusamaMessages OutboundMessages (max_values: None, max_size: Some(2621472), added: 2623947, mode: MaxEncodedLen)
	/// The range of component `x` is `[1, 1000]`.
	pub fn export_message(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `1529`
		// Minimum execution time: 30_323_000 picoseconds.
		Weight::from_parts(32_469_359, 1529)
			// Standard Error: 208
			.saturating_add(Weight::from_parts(176_327, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_710_000 picoseconds.
		Weight::from_parts(2_762_000, 0)
	}
	pub fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_839_000 picoseconds.
		Weight::from_parts(2_931_000, 0)
	}
}
