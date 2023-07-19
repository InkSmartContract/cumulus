use crate::*;
use frame_support::{instances::Instance2, BoundedVec};
use xcm_emulator::{Chain, Parachain};

#[test]
fn swap_locally_on_chain_using_local_assets() {
	const ASSET_ID: u32 = 1;

	let asset_native = Box::new(MultiLocation { parents: 0, interior: Here });
	let asset_one = Box::new(MultiLocation {
		parents: 0,
		interior: X2(PalletInstance(50), GeneralIndex(ASSET_ID.into())),
	});

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::create(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			ASSET_ID.into(),
			AssetHubWestendSender::get().into(),
			1000,
		));
		assert!(<AssetHubWestend as AssetHubWestendPallet>::Assets::asset_exists(ASSET_ID));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::mint(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			ASSET_ID.into(),
			AssetHubWestendSender::get().into(),
			3_000_000_000_000,
		));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::create_pool(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native.clone(),
			asset_one.clone(),
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::PoolCreated { .. }) => {},
			]
		);

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::add_liquidity(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native.clone(),
			asset_one.clone(),
			1_000_000_000_000,
			2_000_000_000_000,
			0,
			0,
			AssetHubWestendSender::get().into()
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::LiquidityAdded {lp_token_minted, .. }) => { lp_token_minted: *lp_token_minted == 1414213562273, },
			]
		);

		let path = BoundedVec::<_, _>::truncate_from(vec![asset_native.clone(), asset_one.clone()]);

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::swap_exact_tokens_for_tokens(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			path,
			100,
			1,
			AssetHubWestendSender::get().into(),
			true
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::SwapExecuted { amount_in, amount_out, .. }) => {
					amount_in: *amount_in == 100,
					amount_out: *amount_out == 199,
				},
			]
		);

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::remove_liquidity(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native,
			asset_one,
			1414213562273 - 2_000_000_000, // all but the 2 EDs can't be retrieved.
			0,
			0,
			AssetHubWestendSender::get().into(),
		));
	});
}

#[test]
fn swap_locally_on_chain_using_foreign_assets() {
	use frame_support::weights::WeightToFee;

	const ASSET_ID: u32 = 1;
	let asset_native = Box::new(MultiLocation { parents: 0, interior: Here });

	let foreign_asset1_at_asset_hub_westend = Box::new(MultiLocation {
		parents: 1,
		interior: X3(
			Parachain(PenpalWestend::para_id().into()),
			PalletInstance(50),
			GeneralIndex(ASSET_ID.into()),
		),
	});

	let assets_para_destination: VersionedMultiLocation =
		MultiLocation { parents: 1, interior: X1(Parachain(AssetHubWestend::para_id().into())) }
			.into();

	let penpal_location =
		MultiLocation { parents: 1, interior: X1(Parachain(PenpalWestend::para_id().into())) };

	// 1. Create asset on penpal:
	PenpalWestend::execute_with(|| {
		assert_ok!(<PenpalWestend as PenpalWestendPallet>::Assets::create(
			<PenpalWestend as Chain>::RuntimeOrigin::signed(PenpalWestendSender::get()),
			ASSET_ID.into(),
			PenpalWestendSender::get().into(),
			1000,
		));

		assert!(<PenpalWestend as PenpalWestendPallet>::Assets::asset_exists(ASSET_ID));
	});

	// 2. Create foreign asset on asset_hub_westend:

	let require_weight_at_most = Weight::from_parts(1_100_000_000_000, 30_000);
	let origin_kind = OriginKind::Xcm;
	let sov_penpal_on_asset_hub_westend = AssetHubWestend::sovereign_account_id_of(penpal_location);

	AssetHubWestend::fund_accounts(vec![
		(AssetHubWestendSender::get(), 5_000_000), // An account to swap dot for something else.
		(sov_penpal_on_asset_hub_westend.clone(), 1000_000_000_000_000_000),
	]);

	let sov_penpal_on_asset_hub_westend_as_location: MultiLocation = MultiLocation {
		parents: 0,
		interior: X1(AccountId32 {
			network: None,
			id: sov_penpal_on_asset_hub_westend.clone().into(),
		}),
	};

	let call_foreign_assets_create =
		<AssetHubWestend as Chain>::RuntimeCall::ForeignAssets(pallet_assets::Call::<
			<AssetHubWestend as Chain>::Runtime,
			Instance2,
		>::create {
			id: *foreign_asset1_at_asset_hub_westend,
			min_balance: 1000,
			admin: sov_penpal_on_asset_hub_westend.clone().into(),
		})
		.encode()
		.into();

	let buy_execution_fee_amount =
		asset_hub_westend_runtime::constants::fee::WeightToFee::weight_to_fee(&Weight::from_parts(
			10_100_000_000_000,
			300_000,
		));
	let buy_execution_fee = MultiAsset {
		id: Concrete(MultiLocation { parents: 1, interior: Here }),
		fun: Fungible(buy_execution_fee_amount),
	};

	let xcm = VersionedXcm::from(Xcm(vec![
		WithdrawAsset { 0: vec![buy_execution_fee.clone()].into() },
		BuyExecution { fees: buy_execution_fee.clone(), weight_limit: Unlimited },
		Transact { require_weight_at_most, origin_kind, call: call_foreign_assets_create },
		RefundSurplus,
		DepositAsset {
			assets: All.into(),
			beneficiary: sov_penpal_on_asset_hub_westend_as_location,
		},
	]));

	// Send XCM message from penpal => asset_hub_westend
	let sudo_penpal_origin = <PenpalWestend as Chain>::RuntimeOrigin::root();
	PenpalWestend::execute_with(|| {
		assert_ok!(<PenpalWestend as PenpalWestendPallet>::PolkadotXcm::send(
			sudo_penpal_origin.clone(),
			bx!(assets_para_destination.clone()),
			bx!(xcm),
		));

		type RuntimeEvent = <PenpalWestend as Chain>::RuntimeEvent;

		assert_expected_events!(
			PenpalWestend,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});

	// Receive XCM message in Assets Parachain
	AssetHubWestend::execute_with(|| {
		assert!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::asset_exists(
			*foreign_asset1_at_asset_hub_westend
		));

		// 3: Mint foreign asset on asset_hub_westend:
		//
		// (While it might be nice to use batch,
		// currently that's disabled due to safe call filters.)

		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;
		// 3. Mint foreign asset (in reality this should be a teleport or some such)
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::ForeignAssets::mint(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(
				sov_penpal_on_asset_hub_westend.clone().into()
			),
			*foreign_asset1_at_asset_hub_westend,
			sov_penpal_on_asset_hub_westend.clone().into(),
			3_000_000_000_000,
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::ForeignAssets(pallet_assets::Event::Issued { .. }) => {},
			]
		);

		// 4. Create pool:
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::create_pool(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native.clone(),
			foreign_asset1_at_asset_hub_westend.clone(),
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::PoolCreated { .. }) => {},
			]
		);

		// 5. Add liquidity:
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::add_liquidity(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(
				sov_penpal_on_asset_hub_westend.clone()
			),
			asset_native.clone(),
			foreign_asset1_at_asset_hub_westend.clone(),
			1_000_000_000_000,
			2_000_000_000_000,
			0,
			0,
			sov_penpal_on_asset_hub_westend.clone().into()
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::LiquidityAdded {lp_token_minted, .. }) => {
					lp_token_minted: *lp_token_minted == 1414213562273,
				},
			]
		);

		// 6. Swap!
		let path = BoundedVec::<_, _>::truncate_from(vec![
			asset_native.clone(),
			foreign_asset1_at_asset_hub_westend.clone(),
		]);

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::swap_exact_tokens_for_tokens(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			path,
			100000,
			1000,
			AssetHubWestendSender::get().into(),
			true
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::SwapExecuted { amount_in, amount_out, .. },) => {
					amount_in: *amount_in == 100000,
					amount_out: *amount_out == 199399,
				},
			]
		);

		// 7. Remove liquidity
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::remove_liquidity(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(
				sov_penpal_on_asset_hub_westend.clone()
			),
			asset_native,
			foreign_asset1_at_asset_hub_westend,
			1414213562273 - 2_000_000_000, // all but the 2 EDs can't be retrieved.
			0,
			0,
			sov_penpal_on_asset_hub_westend.clone().into(),
		));
	});
}

#[test]
fn test_remark_charged_fee() {
	type RuntimeCall = <AssetHubWestend as Chain>::RuntimeCall;
	let genesis_hash = AssetHubWestend::execute_with(|| {
		let genesis_hash = <frame_system::Pallet<asset_hub_westend_runtime::Runtime>>::block_hash(<asset_hub_westend_runtime::Runtime as frame_system::Config>::BlockNumber::from(0u32));
		genesis_hash
	});
	// let genesis_hash = [69u8; 32].into();

	let call = RuntimeCall::System(frame_system::pallet::Call::<_>::remark {
		remark: vec![12u8; 1_000_000_000],
	});
	

	let nonce = 1;
	let extra = //[<$name MultiTxType>]{nonce}.into();
	(
		frame_system::CheckNonZeroSender::<asset_hub_westend_runtime::Runtime>::new(),
		frame_system::CheckSpecVersion::<asset_hub_westend_runtime::Runtime>::new(),
		frame_system::CheckTxVersion::<asset_hub_westend_runtime::Runtime>::new(),
		frame_system::CheckGenesis::<asset_hub_westend_runtime::Runtime>::new(),
		frame_system::CheckEra::<asset_hub_westend_runtime::Runtime>::from(sp_runtime::generic::Era::immortal()),
		frame_system::CheckNonce::<asset_hub_westend_runtime::Runtime>::from(nonce),
		frame_system::CheckWeight::<asset_hub_westend_runtime::Runtime>::new(),
		pallet_asset_conversion_tx_payment::ChargeAssetTxPayment::<asset_hub_westend_runtime::Runtime>::from(
			0, None
		)
	);
	// Is the signed payload from the realy chain???
	// let raw_payload = sp_runtime::generic::SignedPayload::<asset_hub_westend_runtime::RuntimeCall, asset_hub_westend_runtime::SignedExtra>::new(call, extra).unwrap();
	
	let raw_payload = 
	// AssetHubWestend::execute_with(|| {
	// 	sp_runtime::generic::SignedPayload::<asset_hub_westend_runtime::RuntimeCall,
	// 	 asset_hub_westend_runtime::SignedExtra>::new(call, extra).unwrap()
	// });// [69u8; 32].into(),
	sp_runtime::generic::SignedPayload::<asset_hub_westend_runtime::RuntimeCall, asset_hub_westend_runtime::SignedExtra>::from_raw(
			call.clone(),
			extra.clone(),
			(
				(),
				asset_hub_westend_runtime::VERSION.spec_version,
				asset_hub_westend_runtime::VERSION.transaction_version,
				genesis_hash,
				genesis_hash, //TODO: best_hash,
				(),
				(),
				(),
			),
		);
	let sender : sp_keyring::Sr25519Keyring = sp_keyring::Sr25519Keyring::Alice;
	let signature = raw_payload.using_encoded(|e| sender.sign(e));
	let signer: sp_runtime::MultiSigner = sender.public().into();
	 use sp_runtime::traits::IdentifyAccount;
	
	let (call, extra, _) = raw_payload.deconstruct();

	let extrinsic = asset_hub_westend_runtime::UncheckedExtrinsic::new_signed(
		call, 
		signer.into_account().into(),//sp_runtime::AccountId32::from(sender.public()).into(),
		//sp_runtime::MultiSignature::Sr25519(
		signature.into(), 
		extra
	);
	// let extrinsic = asset_hub_westend_runtime::UncheckedExtrinsic::new_signed(call, 
	// 	sp_runtime::AccountId32::from(sender.public()).into(),
	// 	sp_runtime::MultiSignature::Sr25519(signature), 
	// 	extra
	// );

	AssetHubWestend::execute_call(extrinsic);

	// AssetHubWestend::execute_with(|| {
	// 	type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

	// 	assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::System::remark(
	// 		<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestend::account_id_of(
	// 			"random"
	// 		)),
	// 		vec![12u8; 1_000_000_000]
	// 	));
	// });
}

// #[test]
// fn test_remark_charged_fee_should_work_as_have_lots_of_money() {
// 	type RuntimeCall = <AssetHubWestend as Chain>::RuntimeCall;
// 	AssetHubWestend::execute_call(RuntimeCall::System(frame_system::pallet::Call::<_>::remark {
// 		remark: vec![12u8; 1_000_000_000],
// 	}));
// 	AssetHubWestend::execute_with(|| {
// 		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;
//
// 		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::System::remark(
// 			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender),
// 			vec![12u8; 1_000_000_000]
// 		));
// 	});
// }

#[test]
fn transact_while_not_having_any_dot() {
	// Create sufficient asset (xcm sudo request from relay chain to asset hub as no governance on chain)
	const ASSET_ID: u32 = 1;

	let asset_native = Box::new(MultiLocation { parents: 0, interior: Here });
	let asset_one = Box::new(MultiLocation {
		parents: 0,
		interior: X2(PalletInstance(50), GeneralIndex(ASSET_ID.into())),
	});
	//
	// let require_weight_at_most = Weight::from_parts(1_100_000_000_000, 30_000);
	// let origin_kind = OriginKind::Xcm;
	// // let sov_penpal_on_asset_hub_westend = AssetHubWestend::sovereign_account_id_of(penpal_location);
	//
	// Westend::fund_accounts(vec![
	// 	(WestendSender::get(), 5_000_000), // An account to swap dot for something else.
	// 	// (sov_penpal_on_asset_hub_westend.clone(), 1000_000_000_000_000_000),
	// ]);
	//
	// // let sov_penpal_on_asset_hub_westend_as_location: MultiLocation = MultiLocation {
	// // 	parents: 0,
	// // 	interior: X1(AccountId32 {
	// // 		network: None,
	// // 		id: sov_penpal_on_asset_hub_westend.clone().into(),
	// // 	}),
	// // };
	//
	// let call_foreign_assets_create =
	// 	<AssetHubWestend as Para>::RuntimeCall::Assets(pallet_assets::Call::<
	// 		<AssetHubWestend as Para>::Runtime,
	// 		Instance2,
	// 	>::force_create {
	// 		id: *foreign_asset1_at_asset_hub_westend,
	// 		min_balance: 1000,
	// 		sufficient: true,
	// 		admin: AssetHubWestendSender::get().into(),
	// 	})
	// 		.encode()
	// 		.into();
	//
	// let buy_execution_fee_amount =
	// 	asset_hub_westend_runtime::constants::fee::WeightToFee::weight_to_fee(&Weight::from_parts(
	// 		10_100_000_000_000,
	// 		300_000,
	// 	));
	// let buy_execution_fee = MultiAsset {
	// 	id: Concrete(MultiLocation { parents: 1, interior: Here }),
	// 	fun: Fungible(buy_execution_fee_amount),
	// };
	//
	// let xcm = VersionedXcm::from(Xcm(vec![
	// 	WithdrawAsset { 0: vec![buy_execution_fee.clone()].into() },
	// 	BuyExecution { fees: buy_execution_fee.clone(), weight_limit: Unlimited },
	// 	Transact { require_weight_at_most, origin_kind, call: call_foreign_assets_create },
	// 	// RefundSurplus,
	// 	// DepositAsset {
	// 	// 	assets: All.into(),
	// 	// 	beneficiary: sov_penpal_on_asset_hub_westend_as_location,
	// 	// },
	// ]));
	//
	// // Send XCM message from relay => asset_hub_westend
	// let sudo_penpal_origin = <PenpalWestend as Parachain>::RuntimeOrigin::root();
	// PenpalWestend::execute_with(|| {
	// 	assert_ok!(<PenpalWestend as PenpalWestendPallet>::PolkadotXcm::send(
	// 		sudo_penpal_origin.clone(),
	// 		bx!(assets_para_destination.clone()),
	// 		bx!(xcm),
	// 	));
	//
	// 	type RuntimeEvent = <PenpalWestend as Parachain>::RuntimeEvent;
	//
	// 	assert_expected_events!(
	// 		PenpalWestend,
	// 		vec![
	// 			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::Sent { .. }) => {},
	// 		]
	// 	);
	// });

	AssetHubWestend::execute_with(|| {
		type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;

		// 1. Set up a local sufficient asset
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::force_create(
			<AssetHubWestend as Chain>::RuntimeOrigin::root(),
			ASSET_ID.into(),
			AssetHubWestendSender::get().into(),
			true,
			1000,
		));
		assert!(<AssetHubWestend as AssetHubWestendPallet>::Assets::asset_exists(ASSET_ID));

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::mint(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			ASSET_ID.into(),
			AssetHubWestendSender::get().into(),
			3_000_000_000_000,
		));

		// 2. Set up a pool for that asset
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::create_pool(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native.clone(),
			asset_one.clone(),
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::PoolCreated { .. }) => {},
			]
		);

		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::AssetConversion::add_liquidity(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			asset_native.clone(),
			asset_one.clone(),
			1_000_000_000_000,
			2_000_000_000_000,
			0,
			0,
			AssetHubWestendSender::get().into()
		));

		assert_expected_events!(
			AssetHubWestend,
			vec![
				RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::LiquidityAdded {lp_token_minted, .. }) => { lp_token_minted: *lp_token_minted == 1414213562273, },
			]
		);

		// Fund the user's account with the asset:
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::mint(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
			ASSET_ID.into(),
			AssetHubWestend::account_id_of("other").into(),
			3_000_000_000_000,
		));

		// 3. Try and transact having no dot to pay with but only that sufficient asset
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::Assets::transfer(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestend::account_id_of(
				"other"
			)),
			ASSET_ID.into(),
			AssetHubWestendReceiver::get().into(),
			1000,
		));
		// assert!(<AssetHubWestend as AssetHubWestendPallet>::Assets::asset_exists(2));
		// assert_expected_events!(
		// 	AssetHubWestend,
		// 	vec![
		// 		RuntimeEvent::AssetConversion(pallet_asset_conversion::Event::SwapExecuted { amount_in, amount_out, .. }) => {
		// 			amount_in: *amount_in == 100,
		// 			amount_out: *amount_out == 199,
		// 		},
		// 	]
		// );
	});
}
