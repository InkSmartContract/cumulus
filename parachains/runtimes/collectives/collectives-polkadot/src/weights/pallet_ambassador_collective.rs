
//! Autogenerated weights for `pallet_ranked_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-11, STEPS: `20`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --steps=20
// --repeat=1
// --pallet=pallet_ranked_collective
// --extrinsic=*
// --execution=native
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_ranked_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_ranked_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ranked_collective::WeightInfo for WeightInfo<T> {
	// Storage: AmbassadorCollective Members (r:1 w:1)
	// Storage: AmbassadorCollective MemberCount (r:1 w:1)
	// Storage: AmbassadorCollective IndexToId (r:0 w:1)
	// Storage: AmbassadorCollective IdToIndex (r:0 w:1)
	fn add_member() -> Weight {
		// Minimum execution time: 299_000 nanoseconds.
		Weight::from_ref_time(299_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: AmbassadorCollective Members (r:1 w:1)
	// Storage: AmbassadorCollective MemberCount (r:1 w:1)
	// Storage: AmbassadorCollective IdToIndex (r:1 w:1)
	// Storage: AmbassadorCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Minimum execution time: 484_000 nanoseconds.
		Weight::from_ref_time(737_453_843)
			// Standard Error: 28_524_997
			.saturating_add(Weight::from_ref_time(87_120_034).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
	}
	// Storage: AmbassadorCollective Members (r:1 w:1)
	// Storage: AmbassadorCollective MemberCount (r:1 w:1)
	// Storage: AmbassadorCollective IndexToId (r:0 w:1)
	// Storage: AmbassadorCollective IdToIndex (r:0 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Minimum execution time: 358_000 nanoseconds.
		Weight::from_ref_time(385_820_805)
			// Standard Error: 2_396_847
			.saturating_add(Weight::from_ref_time(7_995_427).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: AmbassadorCollective Members (r:1 w:1)
	// Storage: AmbassadorCollective MemberCount (r:1 w:1)
	// Storage: AmbassadorCollective IdToIndex (r:1 w:1)
	// Storage: AmbassadorCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Minimum execution time: 486_000 nanoseconds.
		Weight::from_ref_time(520_244_641)
			// Standard Error: 2_585_811
			.saturating_add(Weight::from_ref_time(13_583_595).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: AmbassadorCollective Members (r:1 w:0)
	// Storage: AmbassadorReferenda ReferendumInfoFor (r:1 w:1)
	// Storage: AmbassadorCollective Voting (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote() -> Weight {
		// Minimum execution time: 660_000 nanoseconds.
		Weight::from_ref_time(660_000_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: AmbassadorReferenda ReferendumInfoFor (r:1 w:0)
	// Storage: AmbassadorCollective VotingCleanup (r:1 w:0)
	// Storage: AmbassadorCollective Voting (r:0 w:5)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Minimum execution time: 243_000 nanoseconds.
		Weight::from_ref_time(293_017_277)
			// Standard Error: 659_221
			.saturating_add(Weight::from_ref_time(12_955_251).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
}
