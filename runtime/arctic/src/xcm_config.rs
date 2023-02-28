use super::{
	AccountId, Assets, Balance, Balances, /*Currencies ,*/ CurrencyId, ParachainInfo, ParachainSystem,
	PolkadotXcm, RelativeCurrencyIdConvert, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
<<<<<<< Updated upstream
	Tokens, Treasury, UnknownTokens, XcmpQueue, UnitWeightCost, MaxInstructions
};
use crate::constants::fee::{ksm_per_second, icz_per_second};
=======
	Tokens, Treasury, UnknownTokens, XcmpQueue, AssetRegistryTrappist
};
use xcm_primitives::AsAssetMultiLocation;
use crate::constants::fee::{ksm_per_second, WeightToFee};
>>>>>>> Stashed changes
use crate::TokenSymbol::*;
use crate::AssetRegistry;
use codec::Encode;
use frame_support::{
	match_types, parameter_types,
	traits::{Everything, Get, Nothing, PalletInfoAccess},
};
<<<<<<< Updated upstream
use orml_traits::{location::AbsoluteReserveProvider, BasicCurrency, MultiCurrency, FixedConversionRateProvider};
use orml_asset_registry::{AssetRegistryTrader, FixedRateAssetRegistryTrader};
=======
use xcm_builder::UsingComponents;
use orml_traits::{BasicCurrency, MultiCurrency};
>>>>>>> Stashed changes
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, AsPrefixedGeneralIndex,
	ConvertedConcreteAssetId, CurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible,
	FixedWeightBounds, FungiblesAdapter, IsConcrete, LocationInverter, ParentAsSuperuser,
	ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue,
	TakeWeightCredit, NativeAsset,
};
use xcm_executor::{
	traits::{FilterAssetLocation, JustTry},
	XcmExecutor,
};
<<<<<<< Updated upstream
=======
use parachains_common::{impls::DealWithFees, AssetId as PCAssetId};
>>>>>>> Stashed changes
use crate::{AdaptedBasicCurrency, NativeCurrencyId};
use orml_xcm_support::{DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset};
use sp_runtime::traits::Convert;

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
    // TODO abhi: what should RelayNetwork be - Any or Kusama?
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	// XCM arctic location
	pub const Local: MultiLocation = Here.into();
	pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
<<<<<<< Updated upstream
    pub TreasuryAccount: AccountId = Treasury::account_id();
=======
    pub SelfReserve: MultiLocation = MultiLocation { parents:0, interior: Here };
>>>>>>> Stashed changes
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<Local>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Balances`.
	(),
>;

/*
pub type LocalAssetTransactor = MultiCurrencyAdapter<
	Currencies,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, RelativeCurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	RelativeCurrencyIdConvert,
	(),
>;
*/

<<<<<<< Updated upstream
pub type DefaultLocalAssetTransactor = MultiCurrencyAdapter<
    Tokens,
    UnknownTokens,
    IsNativeConcrete<CurrencyId, RelativeCurrencyIdConvert>,
    AccountId,
    LocationToAccountId,
    CurrencyId,
    RelativeCurrencyIdConvert,
    DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
=======
/// Means for transacting the native currency on this chain.
pub type DefaultLocalAssetTransactor = CurrencyAdapter<
    // Use this currency:
    Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<SelfReserve>,
	// Convert an XCM MultiLocation into a local account id:
    LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
	// We don't track any teleports of `Balances`.
    ()
>>>>>>> Stashed changes
>;

pub type ReservedFungiblesTransactor = FungiblesAdapter<
	Assets,
	ConvertedConcreteAssetId<
		u128,
		Balance,
		AsAssetMultiLocation<u128, AssetRegistryTrappist>,
		JustTry,
	>,
	LocationToAccountId,
	AccountId,
	Nothing,
	CheckingAccount,
>;

/// Means for transacting assets besides the native currency on this chain.
pub type LocalFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	//ConvertedConcreteAssetId<AssetId, Balance, Local, JustTry>,
	// Use this currency when it is a fungible asset matching the given location or name:
	ConvertedConcreteAssetId<
		u128,
		Balance,
		AsPrefixedGeneralIndex<AssetsPalletLocation, u128, JustTry>,
		JustTry,
	>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Assets`.
	Nothing,
	// We don't track any teleports of `Assets`.
	CheckingAccount,
>;

/// Means for transacting assets on this chain.
// pub type AssetTransactors = (CurrencyTransactor, FungiblesTransactor);
pub type AssetTransactors = (DefaultLocalAssetTransactor, ReservedFungiblesTransactor, LocalFungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub KsmPerSecond: (AssetId, u128) = (MultiLocation::parent().into(), ksm_per_second());
	pub CanonicalizedIczPerSecond: (AssetId, u128) = (
		MultiLocation::new(
			0,
			X1(GeneralKey(ICZ.encode().try_into().unwrap())),
		).into(),
		icz_per_second()
	);
	pub NonCanonicalizedIczPerSecond: (AssetId, u128) = (
		MultiLocation::new(
			1,
			X2(Parachain(ParachainInfo::get().into()), GeneralKey(ICZ.encode().try_into().unwrap())),
		).into(),
		icz_per_second()
	);
}

match_types! {
	pub type ParentOrParentsPlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

pub type XcmBarrier = (
	// Allows local origin messages which call weight_credit >= weight_limit.
	TakeWeightCredit,
	// Allows non-local origin messages, for example from from the xcmp queue,
	// which have the ability to deposit assets and pay for their own execution.
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Parent and its exec plurality get free execution
	AllowUnpaidExecutionFrom<ParentOrParentsPlurality>,
	// Expected responses are OK.
	// Allows `Pending` or `VersionNotifier` query responses.
	AllowKnownQueryResponses<PolkadotXcm>,
	// Subscriptions for version tracking are OK.
	// Allows execution of `SubscribeVersion` or `UnsubscribeVersion` instruction,
	// from parent or sibling chains.
	//AllowSubscriptionsFrom<ParentOrSiblings>,
	AllowSubscriptionsFrom<Everything>,
);

// Allow any asset for reserve on Testnet / dev
pub struct ReserveAssetFilter;
impl FilterAssetLocation for ReserveAssetFilter {
	fn filter_asset_location(_asset: &MultiAsset, _origin: &MultiLocation) -> bool {
		true
	}
}

pub struct ToTreasury;
impl TakeRevenue for ToTreasury {
	fn take_revenue(revenue: MultiAsset) {
		if let MultiAsset {
			id: Concrete(location),
			fun: Fungible(amount),
		} = revenue
		{
			if amount == 0 {
				return;
			}

			if let Some(currency_id) = RelativeCurrencyIdConvert::convert(location) {
                /*
				if currency_id == NativeCurrencyId::get() {
					let _ = AdaptedBasicCurrency::deposit(&TreasuryAccount::get(), amount);
				} else {
					let _ = Tokens::deposit(currency_id, &TreasuryAccount::get(), amount);
				}
                */
                let _ = Tokens::deposit(currency_id, &TreasuryAccount::get(), amount);
			}
		}
	}
}

pub struct MyFixedConversionRateProvider;
impl FixedConversionRateProvider for MyFixedConversionRateProvider {
    fn get_fee_per_second(location: &MultiLocation) -> Option<u128> {
        let metadata = AssetRegistry::fetch_metadata_by_location(location)?;
        Some(metadata.additional.fee_per_second)
    }
}

pub type Trader = (
	FixedRateOfFungible<KsmPerSecond, ToTreasury>,
<<<<<<< Updated upstream
	FixedRateOfFungible<CanonicalizedIczPerSecond, ToTreasury>,
	FixedRateOfFungible<NonCanonicalizedIczPerSecond, ToTreasury>,
    AssetRegistryTrader<FixedRateAssetRegistryTrader<MyFixedConversionRateProvider>, ToTreasury>,
=======
	FixedRateOfFungible<IczPerSecond, ToTreasury>,
    UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, DealWithFees<Runtime>>,
>>>>>>> Stashed changes
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	// type AssetTransactor = DefaultLocalAssetTransactor; //LocalAssetTransactor;  // AssetTransactors;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>; // ReserveAssetFilter; //NativeAsset;
	type IsTeleporter = NativeAsset; // (); // Teleporting is disabled.
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = XcmBarrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = Trader;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type RuntimeOrigin = RuntimeOrigin;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	// ^ Disable dispatchable execute on the XCM pallet.
	// Needs to be `Everything` for local testing.
	type XcmExecutor = XcmExecutor<XcmConfig>;
    // TODO abhi: should XcmTeleportFilter be Nothing or Everything?
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
