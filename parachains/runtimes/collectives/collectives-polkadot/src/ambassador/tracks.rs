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

//! The Ambassador Program's referenda voting tracks.

use super::Origin;
use crate::{
	constants::currency::DOLLARS, Balance, BlockNumber, RuntimeOrigin, DAYS, MINUTES, UNITS,
};
use sp_runtime::Perbill;

/// Referendum `TrackId` type.
pub type TrackId = u16;

/// Referendum track IDs.
pub mod constants {
	use super::TrackId;

	pub const CANDIDATE: TrackId = 0;
	pub const AMBASSADOR_TIER_1: TrackId = 1;
	pub const AMBASSADOR_TIER_2: TrackId = 2;
	pub const SENIOR_AMBASSADOR_TIER_3: TrackId = 3;
	pub const SENIOR_AMBASSADOR_TIER_4: TrackId = 4;
	pub const HEAD_AMBASSADOR_TIER_5: TrackId = 5;
	pub const HEAD_AMBASSADOR_TIER_6: TrackId = 6;
	pub const HEAD_AMBASSADOR_TIER_7: TrackId = 7;
	pub const MASTER_AMBASSADOR_TIER_8: TrackId = 8;
	pub const MASTER_AMBASSADOR_TIER_9: TrackId = 9;
}

/// The type implementing the [`pallet_referenda::TracksInfo`] trait for referenda pallet.
pub struct TracksInfo;

/// Information on the voting tracks.
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = TrackId;

	type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;

	/// Return the array of available tracks and their information.
	fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
		static DATA: [(TrackId, pallet_referenda::TrackInfo<Balance, BlockNumber>); 4] = [
			(
				constants::CANDIDATE,
				pallet_referenda::TrackInfo {
					name: "candidate",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::AMBASSADOR_TIER_1,
				pallet_referenda::TrackInfo {
					name: "ambassador tier 1",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::AMBASSADOR_TIER_2,
				pallet_referenda::TrackInfo {
					name: "ambassador tier 2",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::SENIOR_AMBASSADOR_TIER_3,
				pallet_referenda::TrackInfo {
					name: "senior ambassador tier 3",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::SENIOR_AMBASSADOR_TIER_4,
				pallet_referenda::TrackInfo {
					name: "senior ambassador tier 4",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::HEAD_AMBASSADOR_TIER_5,
				pallet_referenda::TrackInfo {
					name: "head ambassador tier 5",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::HEAD_AMBASSADOR_TIER_6,
				pallet_referenda::TrackInfo {
					name: "head ambassador tier 6",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::HEAD_AMBASSADOR_TIER_7,
				pallet_referenda::TrackInfo {
					name: "head ambassador tier 7",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::MASTER_AMBASSADOR_TIER_8,
				pallet_referenda::TrackInfo {
					name: "master ambassador tier 8",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
			(
				constants::MASTER_AMBASSADOR_TIER_9,
				pallet_referenda::TrackInfo {
					name: "master ambassador tier 9",
					max_deciding: 10,
					decision_deposit: 5 * DOLLARS,
					prepare_period: 30 * MINUTES,
					decision_period: 7 * DAYS,
					confirm_period: 30 * MINUTES,
					min_enactment_period: 5 * MINUTES,
					min_approval: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(50),
						ceil: Perbill::from_percent(100),
					},
					min_support: pallet_referenda::Curve::LinearDecreasing {
						length: Perbill::from_percent(100),
						floor: Perbill::from_percent(10),
						ceil: Perbill::from_percent(50),
					},
				},
			),
		];
		&DATA[..]
	}

	/// Determine the voting track for the given `origin`.
	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		#[cfg(feature = "runtime-benchmarks")]
		{
			// For benchmarks, we enable a root origin.
			// It is important that this is not available in production!
			let root: Self::RuntimeOrigin = frame_system::RawOrigin::Root.into();
			if &root == id {
				return Ok(constants::MASTER_AMBASSADOR_TIER_9)
			}
		}

		match Origin::try_from(id.clone()) {
			Ok(Origin::Candidate) => Ok(constants::CANDIDATE),
			Ok(Origin::AmbassadorTier1) => Ok(constants::AMBASSADOR_TIER_1),
			Ok(Origin::AmbassadorTier2) => Ok(constants::AMBASSADOR_TIER_2),
			Ok(Origin::SeniorAmbassadorTier3) => Ok(constants::SENIOR_AMBASSADOR_TIER_3),
			Ok(Origin::SeniorAmbassadorTier4) => Ok(constants::SENIOR_AMBASSADOR_TIER_4),
			Ok(Origin::HeadAmbassadorTier5) => Ok(constants::HEAD_AMBASSADOR_TIER_5),
			Ok(Origin::HeadAmbassadorTier6) => Ok(constants::HEAD_AMBASSADOR_TIER_6),
			Ok(Origin::HeadAmbassadorTier7) => Ok(constants::HEAD_AMBASSADOR_TIER_7),
			Ok(Origin::MasterAmbassadorTier8) => Ok(constants::MASTER_AMBASSADOR_TIER_8),
			Ok(Origin::MasterAmbassadorTier9) => Ok(constants::MASTER_AMBASSADOR_TIER_9),
			_ => Err(()),
		}
	}
}

// implements [`frame_support::traits::Get`] for [`TracksInfo`]
pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
