// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The Ambassador Program.
//!
//!  TODO update
//! The module defines the following on-chain functionality of the Ambassador Program:
//!
//! - Managed set of program members, where every member has a [rank](ranks) (via
//!   [pallet_ranked_collective]).
//! - Referendum functionality for the program members to propose, vote on, and execute proposals on
//!   behalf of the members of a certain [rank](Origin) (via [pallet_referenda]).
//! - Managed content (charter, announcements) (via [pallet_collective_content]).

pub mod origins;
mod tracks;

use super::*;
use crate::xcm_config::{FellowshipAdminBodyId, UsdtAsset};
use frame_support::traits::{EitherOf, MapSuccess, TryMapSuccess};
pub use origins::pallet_origins as pallet_ambassador_origins;
use origins::pallet_origins::{
	EnsureAmbassadorVoice, EnsureAmbassadorVoiceFrom, EnsureHeadAmbassadorVoice, Origin,
};
use sp_core::ConstU128;
use sp_runtime::traits::{CheckedReduceBy, ConstU16, ConvertToValue, Replace};
use xcm::prelude::*;
use xcm_builder::{AliasesIntoAccountId32, PayOverXcm};

/// The Ambassador Program's member ranks.
pub mod ranks {
	use pallet_ranked_collective::Rank;

	#[allow(dead_code)]
	pub const CANDIDATE: Rank = 0;
	pub const AMBASSADOR_TIER_1: Rank = 1;
	pub const AMBASSADOR_TIER_2: Rank = 2;
	pub const SENIOR_AMBASSADOR_TIER_3: Rank = 3;
	pub const SENIOR_AMBASSADOR_TIER_4: Rank = 4;
	pub const HEAD_AMBASSADOR_TIER_5: Rank = 5;
	pub const HEAD_AMBASSADOR_TIER_6: Rank = 6;
	pub const HEAD_AMBASSADOR_TIER_7: Rank = 7;
	pub const MASTER_AMBASSADOR_TIER_8: Rank = 8;
	pub const MASTER_AMBASSADOR_TIER_9: Rank = 9;
}

impl pallet_ambassador_origins::Config for Runtime {}

pub type AmbassadorCollectiveInstance = pallet_ranked_collective::Instance2;

// Promotion, demotion and approval (rank-retention) is by any of:
// - Root can promote arbitrarily.
// - the FellowshipAdmin origin (i.e. token holder referendum);
// - a senior members vote by the rank two above the new/current rank.
pub type PromoteOrigin = EitherOf<
	frame_system::EnsureRootWithSuccess<AccountId, ConstU16<65535>>,
	EitherOf<
		MapSuccess<
			EnsureXcm<IsVoiceOfBody<GovernanceLocation, FellowshipAdminBodyId>>,
			Replace<ConstU16<{ ranks::MASTER_AMBASSADOR_TIER_9 }>>,
		>,
		TryMapSuccess<
			EnsureAmbassadorVoiceFrom<ConstU16<{ ranks::SENIOR_AMBASSADOR_TIER_3 }>>,
			CheckedReduceBy<ConstU16<2>>,
		>,
	>,
>;

impl pallet_ranked_collective::Config<AmbassadorCollectiveInstance> for Runtime {
	type WeightInfo = (); // TODO actual weights
	type RuntimeEvent = RuntimeEvent;
	type PromoteOrigin = PromoteOrigin;
	type DemoteOrigin = PromoteOrigin;
	type Polls = AmbassadorReferenda;
	type MinRankOfClass = sp_runtime::traits::Identity;
	type VoteWeight = pallet_ranked_collective::Linear;
}

parameter_types! {
	pub const AlarmInterval: BlockNumber = 1;
	pub const SubmissionDeposit: Balance = 0;
	pub const UndecidingTimeout: BlockNumber = 7 * DAYS;
	// The Ambassador Referenda pallet account, used as a temporarily place to deposit a slashed imbalance before teleport to the treasury.
	pub AmbassadorPalletAccount: AccountId = constants::account::AMBASSADOR_REFERENDA_PALLET_ID.into_account_truncating();
}

pub type AmbassadorReferendaInstance = pallet_referenda::Instance2;

impl pallet_referenda::Config<AmbassadorReferendaInstance> for Runtime {
	type WeightInfo = (); // TODO actual weights
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	type Currency = Balances;
	// A proposal can be submitted by a member of the Ambassador Program of [ranks::SENIOR_AMBASSADOR_TIER_3] rank or higher.
	type SubmitOrigin = pallet_ranked_collective::EnsureMember<
		Runtime,
		AmbassadorCollectiveInstance,
		{ ranks::SENIOR_AMBASSADOR_TIER_3 },
	>;
	type CancelOrigin = EitherOf<EnsureRoot<AccountId>, EnsureHeadAmbassadorVoice>;
	type KillOrigin = EitherOf<EnsureRoot<AccountId>, EnsureHeadAmbassadorVoice>;
	type Slash = ToParentTreasury<PolkadotTreasuryAccount, AmbassadorPalletAccount, Runtime>;
	type Votes = pallet_ranked_collective::Votes;
	type Tally = pallet_ranked_collective::TallyOf<Runtime, AmbassadorCollectiveInstance>;
	type SubmissionDeposit = SubmissionDeposit;
	type MaxQueued = ConstU32<20>;
	type UndecidingTimeout = UndecidingTimeout;
	type AlarmInterval = AlarmInterval;
	type Tracks = tracks::TracksInfo;
	type Preimages = Preimage;
}

pub type AmbassadorContentInstance = pallet_collective_content::Instance1;

impl pallet_collective_content::Config<AmbassadorContentInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CharterOrigin = EitherOf<EnsureRoot<AccountId>, EnsureHeadAmbassadorVoice>;
	// An announcement can be submitted by a Senior Ambassador member or an ambassador plurality voice
	// taken via referendum.
	type AnnouncementOrigin = EitherOfDiverse<
		pallet_ranked_collective::EnsureMember<
			Runtime,
			AmbassadorCollectiveInstance,
			{ ranks::SENIOR_AMBASSADOR_TIER_3 },
		>,
		EnsureAmbassadorVoice,
	>;
	type WeightInfo = weights::pallet_collective_content::WeightInfo<Runtime>;
}

pub type AmbassadorCoreInstance = pallet_core_fellowship::Instance2;

impl pallet_core_fellowship::Config<AmbassadorCoreInstance> for Runtime {
	type WeightInfo = (); // TODO actual weights
	type RuntimeEvent = RuntimeEvent;
	type Members = pallet_ranked_collective::Pallet<Runtime, AmbassadorCollectiveInstance>;
	type Balance = Balance;
	// Parameters are set by any of:
	// - Root;
	// - the FellowshipAdmin origin (i.e. token holder referendum);
	// - a vote among all Head Ambassadors.
	type ParamsOrigin = EitherOfDiverse<
		EnsureXcm<IsVoiceOfBody<GovernanceLocation, FellowshipAdminBodyId>>,
		EnsureHeadAmbassadorVoice,
	>;
	// Induction (creating a candidate) is by any of:
	// - Root;
	// - the FellowshipAdmin origin (i.e. token holder referendum);
	// - a single Head Ambassador;
	// - a vote among all senior members.
	type InductOrigin = EitherOfDiverse<
		EnsureXcm<IsVoiceOfBody<GovernanceLocation, FellowshipAdminBodyId>>,
		EitherOfDiverse<
			pallet_ranked_collective::EnsureMember<
				Runtime,
				AmbassadorCollectiveInstance,
				{ ranks::HEAD_AMBASSADOR_TIER_5 },
			>,
			EnsureAmbassadorVoiceFrom<ConstU16<{ ranks::SENIOR_AMBASSADOR_TIER_3 }>>,
		>,
	>;
	type ApproveOrigin = PromoteOrigin;
	type PromoteOrigin = PromoteOrigin;
	type EvidenceSize = ConstU32<65536>;
}

pub type AmbassadorSalaryInstance = pallet_salary::Instance2;

parameter_types! {
	// The interior location on AssetHub for the paying account. This is the Ambassador Salary
	// pallet instance (which sits at index 64). This sovereign account will need funding.
	pub Interior: InteriorMultiLocation = PalletInstance(74).into();
}

const USDT_UNITS: u128 = 1_000_000;

/// [`PayOverXcm`] setup to pay the Ambassador salary on the AssetHub in USDT.
pub type AmbassadorSalaryPaymaster = PayOverXcm<
	Interior,
	crate::xcm_config::XcmRouter,
	crate::PolkadotXcm,
	ConstU32<{ 6 * HOURS }>,
	AccountId,
	(),
	ConvertToValue<UsdtAsset>,
	AliasesIntoAccountId32<(), AccountId>,
>;

impl pallet_salary::Config<AmbassadorSalaryInstance> for Runtime {
	type WeightInfo = weights::pallet_salary::WeightInfo<Runtime>;
	type RuntimeEvent = RuntimeEvent;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type Paymaster = AmbassadorSalaryPaymaster;
	#[cfg(feature = "runtime-benchmarks")]
	type Paymaster = PayWithEnsure<AmbassadorSalaryPaymaster, OpenHrmpChannel<ConstU32<1000>>>;
	type Members = pallet_ranked_collective::Pallet<Runtime, AmbassadorCollectiveInstance>;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type Salary = pallet_core_fellowship::Pallet<Runtime, AmbassadorCoreInstance>;
	#[cfg(feature = "runtime-benchmarks")]
	type Salary = frame_support::traits::tokens::ConvertRank<
		crate::impls::benchmarks::RankToSalary<Balances>,
	>;
	// 15 days to register for a salary payment.
	type RegistrationPeriod = ConstU32<{ 15 * DAYS }>;
	// 15 days to claim the salary payment.
	type PayoutPeriod = ConstU32<{ 15 * DAYS }>;
	// Total monthly salary budget.
	type Budget = ConstU128<{ 100_000 * USDT_UNITS }>;
}