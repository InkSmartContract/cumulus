// Copyright 2022 Parity Technologies (UK) Ltd.
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

//! Bridge definitions.

use crate::{
	BridgeParachainPolkadotInstance, Runtime, WithBridgeHubPolkadotMessagesInstance, XcmRouter,
};
use bp_messages::LaneId;
use bridge_runtime_common::{
	messages,
	messages::{
		source::FromBridgedChainMessagesDeliveryProof, target::FromBridgedChainMessagesProof,
		MessageBridge, ThisChainWithMessages, UnderlyingChainProvider,
	},
	messages_xcm_extension::{XcmBlobHauler, XcmBlobHaulerAdapter},
	refund_relayer_extension::{
		ActualFeeRefund, RefundBridgedParachainMessages, RefundableMessagesLane,
		RefundableParachain,
	},
};
use frame_support::{parameter_types, RuntimeDebug};
use xcm::{latest::prelude::*, prelude::NetworkId};
use xcm_builder::{BridgeBlobDispatcher, HaulBlobExporter};

parameter_types! {
	pub const MaxUnrewardedRelayerEntriesAtInboundLane: bp_messages::MessageNonce =
		bp_bridge_hub_kusama::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX;
	pub const MaxUnconfirmedMessagesAtInboundLane: bp_messages::MessageNonce =
		bp_bridge_hub_kusama::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX;
	pub const BridgeHubPolkadotChainId: bp_runtime::ChainId = bp_runtime::BRIDGE_HUB_POLKADOT_CHAIN_ID;
	pub PolkadotGlobalConsensusNetwork: NetworkId = NetworkId::Polkadot;
	pub ActiveOutboundLanesToBridgeHubPolkadot: &'static [bp_messages::LaneId] = &[DEFAULT_XCM_LANE_TO_BRIDGE_HUB_POLKADOT];
	pub PriorityBoostPerMessage: u64 = 921_900_294;
	pub const BridgeHubPolkadotMessagesLane: bp_messages::LaneId = DEFAULT_XCM_LANE_TO_BRIDGE_HUB_POLKADOT;
	pub const BridgeHubPolkadotParachainId: u32 = {
		use bp_runtime::Parachain;
		BridgeHubPolkadot::PARACHAIN_ID
	};
}

/// Proof of messages, coming from BridgeHubPolkadot.
pub type FromBridgeHubPolkadotMessagesProof =
	FromBridgedChainMessagesProof<bp_bridge_hub_polkadot::Hash>;
/// Messages delivery proof for BridgeHubPolkadot for BridgeHubPolkadot messages.
pub type ToBridgeHubPolkadotMessagesDeliveryProof =
	FromBridgedChainMessagesDeliveryProof<bp_bridge_hub_polkadot::Hash>;

/// Dispatches received XCM messages from other bridge
pub type OnThisChainBlobDispatcher<UniversalLocation> =
	BridgeBlobDispatcher<XcmRouter, UniversalLocation>;

/// Export XCM messages to be relayed to the otherside
pub type ToBridgeHubPolkadotHaulBlobExporter = HaulBlobExporter<
	XcmBlobHaulerAdapter<ToBridgeHubPolkadotXcmBlobHauler>,
	PolkadotGlobalConsensusNetwork,
	(),
>;
pub struct ToBridgeHubPolkadotXcmBlobHauler;
impl XcmBlobHauler for ToBridgeHubPolkadotXcmBlobHauler {
	type MessageSender =
		pallet_bridge_messages::Pallet<Runtime, WithBridgeHubPolkadotMessagesInstance>;

	type MessageSenderOrigin = super::RuntimeOrigin;

	fn message_sender_origin() -> Self::MessageSenderOrigin {
		// TODO:check-parameter - maybe Here.into() is enought?
		pallet_xcm::Origin::from(MultiLocation::new(1, crate::xcm_config::UniversalLocation::get()))
			.into()
	}

	fn xcm_lane() -> LaneId {
		DEFAULT_XCM_LANE_TO_BRIDGE_HUB_POLKADOT
	}
}
pub const DEFAULT_XCM_LANE_TO_BRIDGE_HUB_POLKADOT: LaneId = LaneId([0, 0, 0, 1]);

/// Messaging Bridge configuration for ThisChain -> BridgeHubPolkadot
pub struct WithBridgeHubPolkadotMessageBridge;
impl MessageBridge for WithBridgeHubPolkadotMessageBridge {
	const BRIDGED_MESSAGES_PALLET_NAME: &'static str =
		bp_bridge_hub_kusama::WITH_BRIDGE_HUB_KUSAMA_MESSAGES_PALLET_NAME;
	type ThisChain = ThisChain;
	type BridgedChain = BridgeHubPolkadot;
	type BridgedHeaderChain = pallet_bridge_parachains::ParachainHeaders<
		Runtime,
		BridgeParachainPolkadotInstance,
		bp_bridge_hub_polkadot::BridgeHubPolkadot,
	>;
}

/// Message verifier for BridgeHubPolkadot messages sent from ThisChain
pub type ToBridgeHubPolkadotMessageVerifier =
	messages::source::FromThisChainMessageVerifier<WithBridgeHubPolkadotMessageBridge>;

/// Maximal outbound payload size of ThisChain -> BridgeHubPolkadot messages.
pub type ToBridgeHubPolkadotMaximalOutboundPayloadSize =
	messages::source::FromThisChainMaximalOutboundPayloadSize<WithBridgeHubPolkadotMessageBridge>;

/// BridgeHubPolkadot chain from message lane point of view.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct BridgeHubPolkadot;

impl UnderlyingChainProvider for BridgeHubPolkadot {
	type Chain = bp_bridge_hub_polkadot::BridgeHubPolkadot;
}

impl messages::BridgedChainWithMessages for BridgeHubPolkadot {}

/// ThisChain chain from message lane point of view.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct ThisChain;

impl UnderlyingChainProvider for ThisChain {
	type Chain = bp_bridge_hub_kusama::BridgeHubKusama;
}

impl ThisChainWithMessages for ThisChain {
	type RuntimeOrigin = crate::RuntimeOrigin;
}

/// Signed extension that refunds relayers that are delivering messages from the Polkadot BridgeHub.
pub type BridgeRefundBridgeHubPolkadotMessages = RefundBridgedParachainMessages<
	Runtime,
	RefundableParachain<BridgeParachainPolkadotInstance, BridgeHubPolkadotParachainId>,
	RefundableMessagesLane<WithBridgeHubPolkadotMessagesInstance, BridgeHubPolkadotMessagesLane>,
	ActualFeeRefund<Runtime>,
	PriorityBoostPerMessage,
	StrBridgeRefundBridgeHubPolkadotMessages,
>;
bp_runtime::generate_static_str_provider!(BridgeRefundBridgeHubPolkadotMessages);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::BridgeGrandpaPolkadotInstance;
	use bridge_runtime_common::{
		assert_complete_bridge_types,
		integrity::{
			assert_complete_bridge_constants, check_message_lane_weights,
			AssertBridgeMessagesPalletConstants, AssertBridgePalletNames, AssertChainConstants,
			AssertCompleteBridgeConstants,
		},
	};

	#[test]
	fn ensure_lane_weights_are_correct() {
		check_message_lane_weights::<
			bp_bridge_hub_kusama::BridgeHubKusama,
			Runtime,
			WithBridgeHubPolkadotMessagesInstance,
		>(
			bp_bridge_hub_polkadot::EXTRA_STORAGE_PROOF_SIZE,
			bp_bridge_hub_kusama::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
			bp_bridge_hub_kusama::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX,
			true,
		);
	}

	#[test]
	fn ensure_bridge_integrity() {
		assert_complete_bridge_types!(
			runtime: Runtime,
			with_bridged_chain_grandpa_instance: BridgeGrandpaPolkadotInstance,
			with_bridged_chain_messages_instance: WithBridgeHubPolkadotMessagesInstance,
			bridge: WithBridgeHubPolkadotMessageBridge,
			this_chain: bp_kusama::Kusama,
			bridged_chain: bp_polkadot::Polkadot,
		);

		assert_complete_bridge_constants::<
			Runtime,
			BridgeGrandpaPolkadotInstance,
			WithBridgeHubPolkadotMessagesInstance,
			WithBridgeHubPolkadotMessageBridge,
		>(AssertCompleteBridgeConstants {
			this_chain_constants: AssertChainConstants {
				block_length: bp_bridge_hub_kusama::BlockLength::get(),
				block_weights: bp_bridge_hub_kusama::BlockWeights::get(),
			},
			messages_pallet_constants: AssertBridgeMessagesPalletConstants {
				max_unrewarded_relayers_in_bridged_confirmation_tx:
					bp_bridge_hub_polkadot::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
				max_unconfirmed_messages_in_bridged_confirmation_tx:
					bp_bridge_hub_polkadot::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX,
				bridged_chain_id: bp_runtime::BRIDGE_HUB_POLKADOT_CHAIN_ID,
			},
			pallet_names: AssertBridgePalletNames {
				with_this_chain_messages_pallet_name:
					bp_bridge_hub_kusama::WITH_BRIDGE_HUB_KUSAMA_MESSAGES_PALLET_NAME,
				with_bridged_chain_grandpa_pallet_name:
					bp_polkadot::WITH_POLKADOT_GRANDPA_PALLET_NAME,
				with_bridged_chain_messages_pallet_name:
					bp_bridge_hub_polkadot::WITH_BRIDGE_HUB_POLKADOT_MESSAGES_PALLET_NAME,
			},
		});
	}
}