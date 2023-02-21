use crate::kusama_testnet::*;
use crate::setup::*;
use frame_support::{assert_ok, weights::WeightToFeePolynomial};
use orml_traits::MultiCurrency;
use arctic_runtime::CurrencyId::Token;
use xcm_builder::ParentIsPreset;
use xcm_emulator::TestExt;
use xcm_executor::traits::Convert;

mod hrmp {
    use super::*;

    use polkadot_runtime_parachains::hrmp;
    fn construct_xcm(call: hrmp::Call<kusama_runtime::Runtime>) -> Xcm<()> {
        Xcm(vec![
            WithdrawAsset((Here, 410000000000).into()),
            BuyExecution {
                fees: (Here, 400000000000).into(),
                weight_limit: Unlimited,
            },
            Transact {
                require_weight_at_most: 10000000000,
                origin_type: OriginKind::Native,
                call: kusama_runtime::RuntimeCall::Hrmp(call).encode().into(),
            },
            RefundSurplus,
            DepositAsset {
                assets: All.into(),
                max_assets: 1,
                beneficiary: Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any,
                }
                .into(),
            },
        ])
    }

    fn has_open_channel_requested_event(sender: u32, recipient: u32) -> bool {
        KusamaNet::execute_with(|| {
            kusama_runtime::System::events().iter().any(|r| {
                matches!(
                    r.event,
                    kusama_runtime::RuntimeEvent::Hrmp(hrmp::Event::OpenChannelRequested(
                        actual_sender,
                        actual_recipient,
                        1000,
                        102400
                    )) if actual_sender == sender.into() && actual_recipient == recipient.into()
                )
            })
        })
    }

    fn has_open_channel_accepted_event(sender: u32, recipient: u32) -> bool {
        KusamaNet::execute_with(|| {
            kusama_runtime::System::events().iter().any(|r| {
                matches!(
                    r.event,
                    kusama_runtime::RuntimeEvent::Hrmp(hrmp::Event::OpenChannelAccepted(
                        actual_sender,
                        actual_recipient
                    )) if actual_sender == sender.into() && actual_recipient == recipient.into()
                )
            })
        })
    }

    fn init_open_channel<T>(sender: u32, recipient: u32)
    where
        T: TestExt,
    {
        // do hrmp_init_open_channel
        assert!(!has_open_channel_requested_event(sender, recipient)); // just a sanity check
        T::execute_with(|| {
            let message = construct_xcm(hrmp::Call::<kusama_runtime::Runtime>::hrmp_init_open_channel {
                recipient: recipient.into(),
                proposed_max_capacity: 1000,
                proposed_max_message_size: 102400,
            });
            assert_ok!(pallet_xcm::Pallet::<arctic_runtime::Runtime>::send_xcm(
                Here, Parent, message
            ));
        });
        assert!(has_open_channel_requested_event(sender, recipient));
    }

    fn accept_open_channel<T>(sender: u32, recipient: u32)
    where
        T: TestExt,
    {
        // do hrmp_accept_open_channel
        assert!(!has_open_channel_accepted_event(sender, recipient)); // just a sanity check
        T::execute_with(|| {
            let message = construct_xcm(hrmp::Call::<kusama_runtime::Runtime>::hrmp_accept_open_channel {
                sender: sender.into(),
            });
            assert_ok!(pallet_xcm::Pallet::<arctic_runtime::Runtime>::send_xcm(
                Here, Parent, message
            ));
        });
        assert!(has_open_channel_accepted_event(sender, recipient));
    }
    #[test]
    fn open_hrmp_channel() {
        // setup sovereign account balances
        KusamaNet::execute_with(|| {
            assert_ok!(kusama_runtime::Balances::transfer(
                kusama_runtime::RuntimeOrigin::signed(ALICE.into()),
                sp_runtime::MultiAddress::Id(arctic_sovereign_account_on_kusama()),
                10_820_000_000_000
            ));
            assert_ok!(kusama_runtime::Balances::transfer(
                kusama_runtime::RuntimeOrigin::signed(ALICE.into()),
                sp_runtime::MultiAddress::Id(sibling_sovereign_account_on_kusama()),
                10_820_000_000_000
            ));
        });

        // open channel arctic -> sibling
        init_open_channel::<Arctic>(ARCTIC_PARA_ID, SIBLING_PARA_ID);
        accept_open_channel::<Sibling>(ARCTIC_PARA_ID, SIBLING_PARA_ID);

        // open channel sibling -> arctic
        init_open_channel::<Sibling>(SIBLING_PARA_ID, ARCTIC_PARA_ID);
        accept_open_channel::<Arctic>(SIBLING_PARA_ID, ARCTIC_PARA_ID);

        // check that Bob received left-over funds (from both Arctic and Sibling).
        // We expect slightly less than 4 * 0.41 KSM
        KusamaNet::execute_with(|| {
            let free_balance = kusama_runtime::Balances::free_balance(&AccountId::from(BOB));
            assert!(free_balance > 1_600_000_000_000 && free_balance < 1_640_000_000_000);
        });
    }
}

#[test]
fn transfer_from_relay_chain() {
    init_tracing();
    KusamaNet::execute_with(|| {
        assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
            kusama_runtime::RuntimeOrigin::signed(ALICE.into()),
            Box::new(Parachain(ARCTIC_PARA_ID).into().into()),
            Box::new(
                Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any
                }
                .into()
                .into()
            ),
            Box::new((Here, 2 * 10u128.pow(12)).into()),
            0
        ));
    });

    Arctic::execute_with(|| {
        let xcm_fee = 128_000_000;
        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::KSM), &AccountId::from(BOB)),
            12 * 10u128.pow(12) // - xcm_fee
        );
        assert_eq!(Tokens::free_balance(Token(TokenSymbol::KSM), &TreasuryAccount::get()), xcm_fee);
    });
}

#[test]
fn transfer_to_relay_chain() {
    init_tracing();
    KusamaNet::execute_with(|| {
        assert_ok!(kusama_runtime::Balances::transfer(
            kusama_runtime::RuntimeOrigin::signed(ALICE.into()),
            sp_runtime::MultiAddress::Id(arctic_sovereign_account_on_kusama()),
            2 * 10u128.pow(12)
        ));
    });

    Arctic::execute_with(|| {
        assert_ok!(XTokens::transfer(
            RuntimeOrigin::signed(ALICE.into()),
            Token(TokenSymbol::KSM),
            10u128.pow(12),
            Box::new(
                MultiLocation::new(
                    1,
                    X1(Junction::AccountId32 {
                        id: BOB,
                        network: NetworkId::Any,
                    })
                )
                .into()
            ),
            // WeightLimit::Limited(4_000_000_000) // The value used in UI - very conservative: actually used at time of writing = 298_368_000
            WeightLimit::Unlimited
        ));
    });

    KusamaNet::execute_with(|| {
        let used_weight = 298_368_000; // the actual weight of the sent message
        let fee = 123; // <kusama_runtime::Runtime as pallet_transaction_payment::Config>::WeightToFee::calc(&used_weight);
        assert_eq!(
            kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
            10u128.pow(12) - fee
        );

        // UI uses 165940672 - make sure that that's an overestimation
        assert!(fee < 165940672);
    });
}

/*
/// Send KINT to sibling. On the sibling, it will be registered as a foreign asset.
/// By also transferring it back, we test that the asset-registry has been properly
/// integrated.
#[test]
fn transfer_to_sibling_and_back() {
    fn sibling_sovereign_account() -> AccountId {
        use sp_runtime::traits::AccountIdConversion;
        polkadot_parachain::primitives::Sibling::from(SIBLING_PARA_ID).into_account()
    }

    Sibling::execute_with(|| {
        register_kint_as_foreign_asset();
    });

    Arctic::execute_with(|| {
        assert_ok!(Tokens::deposit(
            Token(TokenSymbol::ICZ),
            &AccountId::from(ALICE),
            100_000_000_000_000
        ));
    });

    Arctic::execute_with(|| {
        assert_ok!(XTokens::transfer(
            RuntimeOrigin::signed(ALICE.into()),
            Token(TokenSymbol::ICZ),
            10_000_000_000_000,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(SIBLING_PARA_ID),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: BOB.into(),
                        }
                    )
                )
                .into()
            ),
            WeightLimit::Limited(1_000_000_000),
        ));

        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::ICZ), &AccountId::from(ALICE)),
            90_000_000_000_000
        );

        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::ICZ), &sibling_sovereign_account()),
            10_000_000_000_000
        );
    });

    Sibling::execute_with(|| {
        let xcm_fee = 800_000_000;

        // check reception
        assert_eq!(
            Tokens::free_balance(ForeignAsset(1), &AccountId::from(BOB)),
            10_000_000_000_000 - xcm_fee
        );

        // return some back to arctic
        assert_ok!(XTokens::transfer(
            RuntimeOrigin::signed(BOB.into()),
            ForeignAsset(1),
            5_000_000_000_000,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(ARCTIC_PARA_ID),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: ALICE.into(),
                        }
                    )
                )
                .into()
            ),
            WeightLimit::Limited(1_000_000_000),
        ));
    });

    // check reception
    Arctic::execute_with(|| {
        let xcm_fee = 170666666;
        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::ICZ), &AccountId::from(ALICE)),
            95_000_000_000_000 - xcm_fee
        );

        assert_eq!(Tokens::free_balance(Token(TokenSymbol::ICZ), &TreasuryAccount::get()), xcm_fee);

        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::ICZ), &sibling_sovereign_account()),
            5_000_000_000_000
        );
    });
}

#[test]
fn xcm_transfer_execution_barrier_trader_works() {
    let expect_weight_limit = 600_000_000;
    let weight_limit_too_low = 500_000_000;
    let unit_instruction_weight = 200_000_000;

    // relay-chain use normal account to send xcm, destination parachain can't pass Barrier check
    let message = Xcm(vec![
        ReserveAssetDeposited((Parent, 100).into()),
        BuyExecution {
            fees: (Parent, 100).into(),
            weight_limit: Unlimited,
        },
        DepositAsset {
            assets: All.into(),
            max_assets: 1,
            beneficiary: Here.into(),
        },
    ]);
    KusamaNet::execute_with(|| {
        // Kusama effectively disabled the `send` extrinsic in 0.9.19, so use send_xcm
        assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
            X1(Junction::AccountId32 {
                network: NetworkId::Any,
                id: ALICE.into(),
            }),
            Parachain(ARCTIC_PARA_ID).into(),
            message
        ));
    });
    Arctic::execute_with(|| {
        assert!(System::events().iter().any(|r| matches!(
            r.event,
            RuntimeEvent::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward(
                _,
                Outcome::Error(XcmError::Barrier)
            ))
        )));
    });

    // AllowTopLevelPaidExecutionFrom barrier test case:
    // para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
    // if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't pass.
    // other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then it's ok.
    let message = Xcm::<arctic_runtime::RuntimeCall>(vec![
        ReserveAssetDeposited((Parent, 100).into()),
        BuyExecution {
            fees: (Parent, 100).into(),
            weight_limit: Limited(weight_limit_too_low),
        },
        DepositAsset {
            assets: All.into(),
            max_assets: 1,
            beneficiary: Here.into(),
        },
    ]);
    Arctic::execute_with(|| {
        let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
        assert_eq!(r, Outcome::Error(XcmError::Barrier));
    });

    // trader inside BuyExecution have TooExpensive error if payment less than calculated weight amount.
    // the minimum of calculated weight amount(`FixedRateOfFungible<KsmPerSecond>`) is 96_000_000
    let message = Xcm::<arctic_runtime::RuntimeCall>(vec![
        ReserveAssetDeposited((Parent, 95_999_999).into()),
        BuyExecution {
            fees: (Parent, 95_999_999).into(),
            weight_limit: Limited(expect_weight_limit),
        },
        DepositAsset {
            assets: All.into(),
            max_assets: 1,
            beneficiary: Here.into(),
        },
    ]);
    Arctic::execute_with(|| {
        let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
        assert_eq!(
            r,
            Outcome::Incomplete(expect_weight_limit - unit_instruction_weight, XcmError::TooExpensive)
        );
    });

    // all situation fulfilled, execute success
    let message = Xcm::<arctic_runtime::RuntimeCall>(vec![
        ReserveAssetDeposited((Parent, 96_000_000).into()),
        BuyExecution {
            fees: (Parent, 96_000_000).into(),
            weight_limit: Limited(expect_weight_limit),
        },
        DepositAsset {
            assets: All.into(),
            max_assets: 1,
            beneficiary: Here.into(),
        },
    ]);
    Arctic::execute_with(|| {
        let r = XcmExecutor::<XcmConfig>::execute_xcm(Parent, message, expect_weight_limit);
        assert_eq!(r, Outcome::Complete(expect_weight_limit));
    });
}

#[test]
fn subscribe_version_notify_works() {
    // relay chain subscribe version notify of para chain
    KusamaNet::execute_with(|| {
        let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::force_subscribe_version_notify(
            kusama_runtime::RuntimeOrigin::root(),
            Box::new(Parachain(ARCTIC_PARA_ID).into().into()),
        );
        assert_ok!(r);
    });
    KusamaNet::execute_with(|| {
        kusama_runtime::System::assert_has_event(kusama_runtime::RuntimeEvent::XcmPallet(
            pallet_xcm::Event::SupportedVersionChanged(
                MultiLocation {
                    parents: 0,
                    interior: X1(Parachain(ARCTIC_PARA_ID)),
                },
                2,
            ),
        ));
    });

    // para chain subscribe version notify of relay chain
    Arctic::execute_with(|| {
        let r = pallet_xcm::Pallet::<arctic_runtime::Runtime>::force_subscribe_version_notify(
            RuntimeOrigin::root(),
            Box::new(Parent.into()),
        );
        assert_ok!(r);
    });
    Arctic::execute_with(|| {
        System::assert_has_event(arctic_runtime::RuntimeEvent::PolkadotXcm(
            pallet_xcm::Event::SupportedVersionChanged(
                MultiLocation {
                    parents: 1,
                    interior: Here,
                },
                2,
            ),
        ));
    });

    // para chain subscribe version notify of sibling chain
    Arctic::execute_with(|| {
        let r = pallet_xcm::Pallet::<arctic_runtime::Runtime>::force_subscribe_version_notify(
            RuntimeOrigin::root(),
            Box::new((Parent, Parachain(SIBLING_PARA_ID)).into()),
        );
        assert_ok!(r);
    });
    Arctic::execute_with(|| {
        assert!(arctic_runtime::System::events().iter().any(|r| matches!(
            r.event,
            arctic_runtime::RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(Some(_)))
        )));
    });
    Sibling::execute_with(|| {
        assert!(testnet_runtime_parachain::System::events().iter().any(|r| matches!(
            r.event,
            testnet_runtime_parachain::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(Some(_)))
                | testnet_runtime_parachain::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success(Some(_)))
        )));
    });
}

#[test]
fn trap_assets_works() {
    let mut kint_treasury_amount = 0;
    let (ksm_asset_amount, kint_asset_amount) = (KSM.one(), KINT.one());
    let trader_weight_to_treasury: u128 = 96_000_000;

    let parent_account: AccountId = ParentIsPreset::<AccountId>::convert(Parent.into()).unwrap();

    Arctic::execute_with(|| {
        assert_ok!(Tokens::deposit(Token(TokenSymbol::KSM), &parent_account, 100 * KSM.one()));
        assert_ok!(Tokens::deposit(Token(TokenSymbol::ICZ), &parent_account, 100 * KINT.one()));

        kint_treasury_amount = Tokens::free_balance(Token(TokenSymbol::ICZ), &TreasuryAccount::get());
    });

    let assets: MultiAsset = (Parent, ksm_asset_amount).into();
    // Withdraw kint and ksm on arctic but don't deposit it
    KusamaNet::execute_with(|| {
        let xcm = vec![
            WithdrawAsset(assets.clone().into()),
            BuyExecution {
                fees: assets,
                weight_limit: Limited(KSM.one() as u64),
            },
            WithdrawAsset(
                (
                    (
                        Parent,
                        X2(Parachain(ARCTIC_PARA_ID), GeneralKey(Token(TokenSymbol::ICZ).encode().try_into().unwrap())),
                    ),
                    kint_asset_amount,
                )
                    .into(),
            ),
        ];
        assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
            Here,
            Parachain(ARCTIC_PARA_ID).into(),
            Xcm(xcm),
        ));
    });

    let mut trapped_assets: Option<MultiAssets> = None;
    // verify that the assets got trapped (i.e. didn't get burned)
    Arctic::execute_with(|| {
        assert!(System::events()
            .iter()
            .any(|r| matches!(r.event, RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _)))));

        let event = System::events()
            .iter()
            .find(|r| matches!(r.event, RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))))
            .cloned()
            .unwrap();

        use std::convert::TryFrom;
        use xcm::VersionedMultiAssets;
        trapped_assets = match event.event {
            RuntimeEvent::PolkadotXcm(pallet_xcm::RuntimeEvent::AssetsTrapped(_, _, ticket)) => {
                Some(TryFrom::<VersionedMultiAssets>::try_from(ticket).unwrap())
            }
            _ => panic!("event not found"),
        };

        // unchanged treasury amounts
        assert_eq!(
            trader_weight_to_treasury,
            Tokens::free_balance(Token(TokenSymbol::KSM), &TreasuryAccount::get())
        );
        assert_eq!(
            kint_treasury_amount,
            Tokens::free_balance(Token(TokenSymbol::ICZ), &TreasuryAccount::get())
        );
    });

    // Now reclaim trapped assets
    KusamaNet::execute_with(|| {
        let xcm = vec![
            ClaimAsset {
                assets: trapped_assets.unwrap(),
                ticket: Here.into(),
            },
            BuyExecution {
                fees: (
                    (
                        Parent,
                        X2(Parachain(ARCTIC_PARA_ID), GeneralKey(Token(TokenSymbol::ICZ).encode().try_into().unwrap())),
                    ),
                    kint_asset_amount / 2,
                )
                    .into(),
                weight_limit: Limited(KSM.one() as u64),
            },
            DepositAsset {
                assets: All.into(),
                max_assets: 2,
                beneficiary: Junction::AccountId32 {
                    id: BOB,
                    network: NetworkId::Any,
                }
                .into(),
            },
        ];
        assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
            Here,
            Parachain(ARCTIC_PARA_ID).into(),
            Xcm(xcm),
        ));
    });

    // verify that assets were claimed successfully (deposited into Bob's account)
    Arctic::execute_with(|| {
        let kint_xcm_fee = 127_999_999;
        let ksm_xcm_fee = 96_000_000;
        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::ICZ), &AccountId::from(BOB)),
            kint_asset_amount - kint_xcm_fee
        );
        assert_eq!(
            Tokens::free_balance(Token(TokenSymbol::KSM), &AccountId::from(BOB)),
            ksm_asset_amount - ksm_xcm_fee
        );
    });
}
*/

fn register_kint_as_foreign_asset() {
    let metadata = orml_asset_registry::AssetMetadata {
        decimals: 12,
        name: "Arctic native".as_bytes().to_vec(),
        symbol: "extICZ".as_bytes().to_vec(),
        existential_deposit: 0,
        location: Some(MultiLocation::new(1, X2(Parachain(ARCTIC_PARA_ID), GeneralKey(Token(TokenSymbol::ICZ).encode().try_into().unwrap()))).into()),
        additional: CustomMetadata {
            fee_per_second: 1_000_000_000_000,
        },
    };
    AssetRegistry::register_asset(RuntimeOrigin::root(), metadata, None).unwrap();
}
