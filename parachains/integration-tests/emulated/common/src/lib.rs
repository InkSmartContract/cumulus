// #![feature(trivial_bounds)]
pub mod constants;

pub use constants::{
	accounts::{ALICE, BOB},
	asset_hub_kusama, asset_hub_polkadot, asset_hub_westend, bridge_hub_kusama,
	bridge_hub_polkadot, collectives, kusama, penpal, polkadot, westend,
};
use frame_support::{parameter_types, sp_io, sp_tracing};
pub use parachains_common::{AccountId, AssetHubPolkadotAuraId, AuraId, Balance, BlockNumber};
pub use sp_core::{sr25519, storage::Storage, Get};
use xcm::prelude::*;
use xcm_emulator::{
	decl_test_networks, decl_test_parachains, decl_test_relay_chains, Chain, Parachain, RelayChain,
	TestExt,
};
use polkadot_parachain_node::service::*;
use xcm_executor::traits::ConvertLocation;

decl_test_relay_chains! {
	#[api_version(5)]
	pub struct Westend {
		genesis = westend::genesis(),
		on_init = (),
		runtime = {
			Runtime: westend_runtime::Runtime,
			RuntimeOrigin: westend_runtime::RuntimeOrigin,
			RuntimeCall: westend_runtime::RuntimeCall,
			RuntimeEvent: westend_runtime::RuntimeEvent,
			MessageQueue: westend_runtime::MessageQueue,
			XcmConfig: westend_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: westend_runtime::xcm_config::LocationConverter, //TODO: rename to SovereignAccountOf,
			System: westend_runtime::System,
			Balances: westend_runtime::Balances,
			UncheckedExtrinsic: westend_runtime::UncheckedExtrinsic,
			Block: westend_runtime::Block,
			RuntimeApi: westend_runtime::RuntimeApi,
			SignedExtra: westend_runtime::SignedExtra,
		},
		pallets_extra = {
			XcmPallet: westend_runtime::XcmPallet,
			Sudo: westend_runtime::Sudo,
		}
	},
	#[api_version(5)]
	pub struct Polkadot {
		genesis = polkadot::genesis(),
		on_init = (),
		runtime = {
			Runtime: polkadot_runtime::Runtime,
			RuntimeOrigin: polkadot_runtime::RuntimeOrigin,
			RuntimeCall: polkadot_runtime::RuntimeCall,
			RuntimeEvent: polkadot_runtime::RuntimeEvent,
			MessageQueue: polkadot_runtime::MessageQueue,
			XcmConfig: polkadot_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: polkadot_runtime::xcm_config::SovereignAccountOf,
			System: polkadot_runtime::System,
			Balances: polkadot_runtime::Balances,
			UncheckedExtrinsic: polkadot_runtime::UncheckedExtrinsic,
			Block: polkadot_runtime::Block,
			RuntimeApi: polkadot_runtime::RuntimeApi,
			SignedExtra: polkadot_runtime::SignedExtra,
		},
		pallets_extra = {
			XcmPallet: polkadot_runtime::XcmPallet,
		}
	},
	#[api_version(5)]
	pub struct Kusama {
		genesis = kusama::genesis(),
		on_init = (),
		runtime = {
			Runtime: kusama_runtime::Runtime,
			RuntimeOrigin: kusama_runtime::RuntimeOrigin,
			RuntimeCall: kusama_runtime::RuntimeCall,
			RuntimeEvent: kusama_runtime::RuntimeEvent,
			MessageQueue: kusama_runtime::MessageQueue,
			XcmConfig: kusama_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: kusama_runtime::xcm_config::SovereignAccountOf,
			System: kusama_runtime::System,
			Balances: kusama_runtime::Balances,
			UncheckedExtrinsic: kusama_runtime::UncheckedExtrinsic,
			Block: kusama_runtime::Block,
			RuntimeApi: kusama_runtime::RuntimeApi,
			SignedExtra: kusama_runtime::SignedExtra,
		},
		pallets_extra = {
			XcmPallet: kusama_runtime::XcmPallet,
		}
	}
}

decl_test_parachains! {
	// Westend
	pub struct AssetHubWestend {
		genesis = asset_hub_westend::genesis(),
		on_init = (),
		runtime = {
			Runtime: asset_hub_westend_runtime::Runtime,
			RuntimeOrigin: asset_hub_westend_runtime::RuntimeOrigin,
			RuntimeCall: asset_hub_westend_runtime::RuntimeCall,
			RuntimeEvent: asset_hub_westend_runtime::RuntimeEvent,
			XcmpMessageHandler: asset_hub_westend_runtime::XcmpQueue,
			DmpMessageHandler: asset_hub_westend_runtime::DmpQueue,
			LocationToAccountId: asset_hub_westend_runtime::xcm_config::LocationToAccountId,
			System: asset_hub_westend_runtime::System,
			Balances: asset_hub_westend_runtime::Balances,
			ParachainSystem: asset_hub_westend_runtime::ParachainSystem,
			ParachainInfo: asset_hub_westend_runtime::ParachainInfo,
			UncheckedExtrinsic: asset_hub_westend_runtime::UncheckedExtrinsic,
			Block: asset_hub_westend_runtime::Block,
			RuntimeApi: asset_hub_westend_runtime::RuntimeApi,
			SignedExtra: asset_hub_westend_runtime::SignedExtra,
		},
		pallets_extra = {
			System: asset_hub_westend_runtime::System,

			TransactionPayment: asset_hub_westend_runtime::TransactionPayment,
			AssetTxPayment: asset_hub_westend_runtime::AssetTxPayment,

			PolkadotXcm: asset_hub_westend_runtime::PolkadotXcm,
			Assets: asset_hub_westend_runtime::Assets,


			ForeignAssets: asset_hub_westend_runtime::ForeignAssets,
			PoolAssets: asset_hub_westend_runtime::PoolAssets,
			AssetConversion: asset_hub_westend_runtime::AssetConversion,
		}
	},
	// Polkadot
	pub struct AssetHubPolkadot {
		genesis = asset_hub_polkadot::genesis(),
		on_init = (),
		runtime = {
			Runtime: asset_hub_polkadot_runtime::Runtime,
			RuntimeOrigin: asset_hub_polkadot_runtime::RuntimeOrigin,
			RuntimeCall: asset_hub_polkadot_runtime::RuntimeCall,
			RuntimeEvent: asset_hub_polkadot_runtime::RuntimeEvent,
			XcmpMessageHandler: asset_hub_polkadot_runtime::XcmpQueue,
			DmpMessageHandler: asset_hub_polkadot_runtime::DmpQueue,
			LocationToAccountId: asset_hub_polkadot_runtime::xcm_config::LocationToAccountId,
			System: asset_hub_polkadot_runtime::System,
			Balances: asset_hub_polkadot_runtime::Balances,
			ParachainSystem: asset_hub_polkadot_runtime::ParachainSystem,
			ParachainInfo: asset_hub_polkadot_runtime::ParachainInfo,
			UncheckedExtrinsic: asset_hub_polkadot_runtime::UncheckedExtrinsic,
			Block: asset_hub_polkadot_runtime::Block,
			RuntimeApi: asset_hub_polkadot_runtime::RuntimeApi,
			SignedExtra: asset_hub_polkadot_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: asset_hub_polkadot_runtime::PolkadotXcm,
			Assets: asset_hub_polkadot_runtime::Assets,
		}
	},
	pub struct PenpalPolkadot {
		genesis = penpal::genesis(penpal::PARA_ID),
		on_init = (),
		runtime = {
			Runtime: penpal_runtime::Runtime,
			RuntimeOrigin: penpal_runtime::RuntimeOrigin,
			RuntimeCall: penpal_runtime::RuntimeCall,
			RuntimeEvent: penpal_runtime::RuntimeEvent,
			XcmpMessageHandler: penpal_runtime::XcmpQueue,
			DmpMessageHandler: penpal_runtime::DmpQueue,
			LocationToAccountId: penpal_runtime::xcm_config::LocationToAccountId,
			System: penpal_runtime::System,
			Balances: penpal_runtime::Balances,
			ParachainSystem: penpal_runtime::ParachainSystem,
			ParachainInfo: penpal_runtime::ParachainInfo,
			UncheckedExtrinsic: penpal_runtime::UncheckedExtrinsic,
			Block: penpal_runtime::Block,
			RuntimeApi: penpal_runtime::RuntimeApi,
			SignedExtra: penpal_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: penpal_runtime::PolkadotXcm,
			Assets: penpal_runtime::Assets,
		}
	},
	pub struct PenpalWestend {
		genesis = penpal::genesis(penpal::PARA_ID),
		on_init = (),
		runtime = {
			Runtime: penpal_runtime::Runtime,
			RuntimeOrigin: penpal_runtime::RuntimeOrigin,
			RuntimeCall: penpal_runtime::RuntimeCall,
			RuntimeEvent: penpal_runtime::RuntimeEvent,
			XcmpMessageHandler: penpal_runtime::XcmpQueue,
			DmpMessageHandler: penpal_runtime::DmpQueue,
			LocationToAccountId: penpal_runtime::xcm_config::LocationToAccountId,
			System: penpal_runtime::System,
			Balances: penpal_runtime::Balances,
			ParachainSystem: penpal_runtime::ParachainSystem,
			ParachainInfo: penpal_runtime::ParachainInfo,
			UncheckedExtrinsic: penpal_runtime::UncheckedExtrinsic,
			Block: penpal_runtime::Block,
			RuntimeApi: penpal_runtime::RuntimeApi,
			SignedExtra: penpal_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: penpal_runtime::PolkadotXcm,
			Assets: penpal_runtime::Assets,
		}
	},

	// Kusama
	pub struct AssetHubKusama {
		genesis = asset_hub_kusama::genesis(),
		on_init = (),
		runtime = {
			Runtime: asset_hub_kusama_runtime::Runtime,
			RuntimeOrigin: asset_hub_kusama_runtime::RuntimeOrigin,
			RuntimeCall: asset_hub_kusama_runtime::RuntimeCall,
			RuntimeEvent: asset_hub_kusama_runtime::RuntimeEvent,
			XcmpMessageHandler: asset_hub_kusama_runtime::XcmpQueue,
			DmpMessageHandler: asset_hub_kusama_runtime::DmpQueue,
			LocationToAccountId: asset_hub_kusama_runtime::xcm_config::LocationToAccountId,
			System: asset_hub_kusama_runtime::System,
			Balances: asset_hub_kusama_runtime::Balances,
			ParachainSystem: asset_hub_kusama_runtime::ParachainSystem,
			ParachainInfo: asset_hub_kusama_runtime::ParachainInfo,
			UncheckedExtrinsic: asset_hub_kusama_runtime::UncheckedExtrinsic,
			Block: asset_hub_kusama_runtime::Block,
			RuntimeApi: asset_hub_kusama_runtime::RuntimeApi,
			SignedExtra: asset_hub_kusama_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: asset_hub_kusama_runtime::PolkadotXcm,
			Assets: asset_hub_kusama_runtime::Assets,
			ForeignAssets: asset_hub_kusama_runtime::Assets,
		}
	},
	pub struct PenpalKusama {
		genesis = penpal::genesis(penpal::PARA_ID),
		on_init = (),
		runtime = {
			Runtime: penpal_runtime::Runtime,
			RuntimeOrigin: penpal_runtime::RuntimeOrigin,
			RuntimeCall: penpal_runtime::RuntimeCall,
			RuntimeEvent: penpal_runtime::RuntimeEvent,
			XcmpMessageHandler: penpal_runtime::XcmpQueue,
			DmpMessageHandler: penpal_runtime::DmpQueue,
			LocationToAccountId: penpal_runtime::xcm_config::LocationToAccountId,
			System: penpal_runtime::System,
			Balances: penpal_runtime::Balances,
			ParachainSystem: penpal_runtime::ParachainSystem,
			ParachainInfo: penpal_runtime::ParachainInfo,
			UncheckedExtrinsic: penpal_runtime::UncheckedExtrinsic,
			Block: penpal_runtime::Block,
			RuntimeApi: penpal_runtime::RuntimeApi,
			SignedExtra: penpal_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: penpal_runtime::PolkadotXcm,
			Assets: penpal_runtime::Assets,
		}
	},
	pub struct CollectivesPolkadot {
		genesis = collectives::genesis(),
		on_init = (),
		runtime = {
			Runtime: collectives_polkadot_runtime::Runtime,
			RuntimeOrigin: collectives_polkadot_runtime::RuntimeOrigin,
			RuntimeCall: collectives_polkadot_runtime::RuntimeCall,
			RuntimeEvent: collectives_polkadot_runtime::RuntimeEvent,
			XcmpMessageHandler: collectives_polkadot_runtime::XcmpQueue,
			DmpMessageHandler: collectives_polkadot_runtime::DmpQueue,
			LocationToAccountId: collectives_polkadot_runtime::xcm_config::LocationToAccountId,
			System: collectives_polkadot_runtime::System,
			Balances: collectives_polkadot_runtime::Balances,
			ParachainSystem: collectives_polkadot_runtime::ParachainSystem,
			ParachainInfo: collectives_polkadot_runtime::ParachainInfo,
			UncheckedExtrinsic: collectives_polkadot_runtime::UncheckedExtrinsic,
			Block: collectives_polkadot_runtime::Block,
			RuntimeApi: collectives_polkadot_runtime::RuntimeApi,
			SignedExtra: collectives_polkadot_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: collectives_polkadot_runtime::PolkadotXcm,
		}
	},
	pub struct BHKusama {
		genesis = bridge_hub_kusama::genesis(),
		on_init = (),
		runtime = {
			Runtime: bridge_hub_kusama_runtime::Runtime,
			RuntimeOrigin: bridge_hub_kusama_runtime::RuntimeOrigin,
			RuntimeCall: bridge_hub_kusama_runtime::RuntimeCall,
			RuntimeEvent: bridge_hub_kusama_runtime::RuntimeEvent,
			XcmpMessageHandler: bridge_hub_kusama_runtime::XcmpQueue,
			DmpMessageHandler: bridge_hub_kusama_runtime::DmpQueue,
			LocationToAccountId: bridge_hub_kusama_runtime::xcm_config::LocationToAccountId,
			System: bridge_hub_kusama_runtime::System,
			Balances: bridge_hub_kusama_runtime::Balances,
			ParachainSystem: bridge_hub_kusama_runtime::ParachainSystem,
			ParachainInfo:bridge_hub_kusama_runtime::ParachainInfo,
			UncheckedExtrinsic: bridge_hub_kusama_runtime::UncheckedExtrinsic,
			Block: bridge_hub_kusama_runtime::Block,
			RuntimeApi: bridge_hub_kusama_runtime::RuntimeApi,
			SignedExtra: bridge_hub_kusama_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: bridge_hub_kusama_runtime::PolkadotXcm,
		}
	},
	pub struct BHPolkadot {
		genesis = bridge_hub_polkadot::genesis(),
		on_init = (),
		runtime = {
			Runtime: bridge_hub_polkadot_runtime::Runtime,
			RuntimeOrigin: bridge_hub_polkadot_runtime::RuntimeOrigin,
			RuntimeCall: bridge_hub_polkadot_runtime::RuntimeCall,
			RuntimeEvent: bridge_hub_polkadot_runtime::RuntimeEvent,
			XcmpMessageHandler: bridge_hub_polkadot_runtime::XcmpQueue,
			DmpMessageHandler: bridge_hub_polkadot_runtime::DmpQueue,
			LocationToAccountId: bridge_hub_polkadot_runtime::xcm_config::LocationToAccountId,
			System: bridge_hub_polkadot_runtime::System,
			Balances: bridge_hub_polkadot_runtime::Balances,
			ParachainSystem: bridge_hub_polkadot_runtime::ParachainSystem,
			ParachainInfo:bridge_hub_polkadot_runtime::ParachainInfo,
			UncheckedExtrinsic: bridge_hub_polkadot_runtime::UncheckedExtrinsic,
			Block: bridge_hub_polkadot_runtime::Block,
			RuntimeApi: bridge_hub_polkadot_runtime::RuntimeApi,
			SignedExtra: bridge_hub_polkadot_runtime::SignedExtra,
		},
		pallets_extra = {
			PolkadotXcm: bridge_hub_polkadot_runtime::PolkadotXcm,
		}
	}
}

decl_test_networks! {
	pub struct PolkadotMockNet {
		relay_chain = Polkadot,
		parachains = vec![
			AssetHubPolkadot,
			PenpalPolkadot,
			CollectivesPolkadot,
			BHPolkadot,
		],
	},
	pub struct KusamaMockNet {
		relay_chain = Kusama,
		parachains = vec![
			AssetHubKusama,
			PenpalKusama,
			BHKusama,
		],
	},
	pub struct WestendMockNet {
		relay_chain = Westend,
		parachains = vec![
			AssetHubWestend,
			PenpalWestend,
		],
	}
}

parameter_types! {
	// Polkadot
	pub PolkadotSender: AccountId = Polkadot::account_id_of(ALICE);
	pub PolkadotReceiver: AccountId = Polkadot::account_id_of(BOB);
	// Kusama
	pub KusamaSender: AccountId = Kusama::account_id_of(ALICE);
	pub KusamaReceiver: AccountId = Kusama::account_id_of(BOB);
	// Westend
	pub WestendSender: AccountId = Westend::account_id_of(ALICE);
	pub WestendReceiver: AccountId = Westend::account_id_of(BOB);
	// Asset Hub Westend
	pub AssetHubWestendSender: AccountId = AssetHubWestend::account_id_of(ALICE);
	pub AssetHubWestendReceiver: AccountId = AssetHubWestend::account_id_of(BOB);
	// Asset Hub Polkadot
	pub AssetHubPolkadotSender: AccountId = AssetHubPolkadot::account_id_of(ALICE);
	pub AssetHubPolkadotReceiver: AccountId = AssetHubPolkadot::account_id_of(BOB);
	// Asset Hub Kusama
	pub AssetHubKusamaSender: AccountId = AssetHubKusama::account_id_of(ALICE);
	pub AssetHubKusamaReceiver: AccountId = AssetHubKusama::account_id_of(BOB);
	// Penpal Polkadot
	pub PenpalPolkadotSender: AccountId = PenpalPolkadot::account_id_of(ALICE);
	pub PenpalPolkadotReceiver: AccountId = PenpalPolkadot::account_id_of(BOB);
	// Penpal Kusama
	pub PenpalKusamaSender: AccountId = PenpalKusama::account_id_of(ALICE);
	pub PenpalKusamaReceiver: AccountId = PenpalKusama::account_id_of(BOB);
	// Penpal Westend
	pub PenpalWestendSender: AccountId = PenpalWestend::account_id_of(ALICE);
	pub PenpalWestendReceiver: AccountId = PenpalWestend::account_id_of(BOB);
	// Collectives
	pub CollectivesSender: AccountId = CollectivesPolkadot::account_id_of(ALICE);
	pub CollectivesReceiver: AccountId = CollectivesPolkadot::account_id_of(BOB);
	// Bridge Hub Polkadot
	pub BHPolkadotSender: AccountId = BHPolkadot::account_id_of(ALICE);
	pub BHPolkadotReceiver: AccountId = BHPolkadot::account_id_of(BOB);
	// Bridge Hub Kusama
	pub BHKusamaSender: AccountId = BHKusama::account_id_of(ALICE);
	pub BHKusamaReceiver: AccountId = BHKusama::account_id_of(BOB);
}




// macro_rules! generate {
// 	($runtime_name:ident, $runtime_namespace:path) => {
// 		paste::paste! {
// 			#[derive(Clone)]
// 			pub struct [<$runtime_name Executor>];

// 			impl sc_executor::NativeExecutionDispatch for [<$runtime_name Executor>] {
// 				type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

// 				fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
// 					$runtime_namespace::api::dispatch(method, data)
// 				}

// 				fn native_version() -> sc_executor::NativeVersion {
// 					$runtime_namespace::native_version()
// 				}
// 			}
// 		}
// 	};
// }

// generate!(BHPolkadot, bridge_hub_polkadot_runtime);
// generate!(BHKusama, bridge_hub_kusama_runtime);
// generate!(Collectives, collectives_polkadot_runtime);
// generate!(PenpalKusama, penpal_runtime);
// generate!(PenpalPolkadot, penpal_runtime);
// generate!(PenpalWestend, penpal_runtime);
// generate!(AssetHubPolkadot, asset_hub_polkadot_runtime);
// generate!(AssetHubKusama, asset_hub_kusama_runtime);
// generate!(AssetHubWestend, asset_hub_westend_runtime);