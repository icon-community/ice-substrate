//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(feature = "std")]
/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect(
		"Development wasm binary is not available. This means the client is \
                        built with `BUILD_DUMMY_WASM_BINARY` flag and it is only usable for \
                        production chains. Please rebuild with the flag disabled.",
	)
}

use codec::{Decode, Encode};
use pallet_evm::FeeCalculator;


use frame_system::limits::{BlockLength, BlockWeights};
use frame_system::EnsureRoot;
use sp_api::impl_runtime_apis;
use sp_core::{
	crypto::{KeyTypeId},
	OpaqueMetadata, H160, H256, U256,
};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, Dispatchable, IdentifyAccount,
		PostDispatchInfoOf, Verify, ConvertInto, Zero, Convert, AccountIdConversion
	},
	DispatchError,
	transaction_validity::{TransactionSource, TransactionValidity, TransactionValidityError},
	ApplyExtrinsicResult, FixedPointNumber, MultiSignature, Perquintill,
};

use sp_std::{marker::PhantomData, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use orml_asset_registry;

use frame_support::inherent::Vec;
use sp_std::boxed::Box;

pub mod constants;
pub mod impls;

use constants::{currency, fee, paras, time};
use currency::*;
use fee::*;
use impls::*;
use time::*;
pub use impls::DealWithFees;

pub type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;


// XCM Imports

use pallet_traits::{
    xcm::{
        AccountIdToMultiLocation, AsAssetType, AssetType, CurrencyIdtoMultiLocation,
        FirstAssetTrader, MultiCurrencyAdapter,
    },
    DecimalProvider, EmergencyCallFilter, ValidationDataProvider,
};

use xcm::latest::prelude::BodyId;

use cumulus_primitives_core::ParaId as CumulusParaId;
use crate::currency::dot_per_second;

use orml_traits::{
    location::AbsoluteReserveProvider, parameter_type_with_key, DataFeeder, DataProvider,
    DataProviderExtended,
};
use orml_xcm_support::{IsNativeConcrete, MultiNativeAsset};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, EnsureXcmOrigin, ConvertedConcreteAssetId, FixedRateOfFungible,
	FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset, ParentAsSuperuser, FungiblesAdapter,
	ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
	UsingComponents, TakeRevenue
};

use primitives::{
    tokens::{ACA, AUSD, DOT, EUSDC, EUSDT, LC_DOT, LDOT, ICZ, ICX, KAR},
    AccountId, AuraId, Balance, BlockNumber, CurrencyId, DataProviderId, EraIndex, Hash,
    Index, Liquidity, Moment, ParaId, PersistedValidationData, Price, Rate, Ratio, Shortfall,
    Signature,
};

use xcm_executor::{traits::JustTry, Config, XcmExecutor};


// xcm import ends

// A few exports that help ease life for downstream crates.
use fp_rpc::TransactionStatus;
pub use frame_support::{
	construct_runtime, parameter_types, match_types,
	traits::{
		fungibles::{InspectMetadata, Mutate},
		tokens::BalanceConversion,
		FindAuthor, KeyOwnerProofSystem, Randomness, Currency,
		ChangeMembers, Contains, EnsureOneOf, EqualPrivilegeOnly, Everything, InstanceFilter,
        Nothing,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		ConstantMultiplier, DispatchClass, Weight,
	},
	ConsensusEngineId, PalletId, StorageValue,
};

pub use pallet_balances::Call as BalancesCall;
use pallet_contracts::weights::WeightInfo;
use pallet_ethereum::{Call::transact, Transaction as EthereumTransaction};
use pallet_evm::{Account as EVMAccount, EnsureAddressTruncated, HashedAddressMapping, Runner};
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

mod precompile;
use precompile::FrontierPrecompiles;


/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Digest item type.
// pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("arctic-testnet"),
	impl_name: create_runtime_str!("arctic-testnet"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

// Prints debug output of the `contracts` pallet to stdout if the node is
// started with `-lruntime::contracts=debug`.
const CONTRACTS_DEBUG_OUTPUT: bool = true;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 256;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub RuntimeBlockWeights: BlockWeights = BlockWeights
			::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockLength: BlockLength = BlockLength
			::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;

	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;

	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account();
 }

 
parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const MinCandidates: u32 = 5;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = frame_system::EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinCandidates = MinCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

parameter_types! {
	pub const DepositPerItem: Balance = currency::deposit(1, 0);
	pub const DepositPerByte: Balance = currency::deposit(0, 1);
	// The lazy deletion runs inside on_initialize.
	pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
		RuntimeBlockWeights::get().max_block;
	// The weight needed for decoding the queue should be less or equal than a fifth
	// of the overall weight dedicated to the lazy deletion.
	pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
		)) / 5) as u32;
	pub Schedule: pallet_contracts::Schedule<Runtime> = {
		let mut schedule = pallet_contracts::Schedule::<Runtime>::default();
		// We decided to **temporarily* increase the default allowed contract size here
		// (the default is `128 * 1024`).
		//
		// Our reasoning is that a number of people ran into `CodeTooLarge` when trying
		// to deploy their contracts. We are currently introducing a number of optimizations
		// into ink! which should bring the contract sizes lower. In the meantime we don't
		// want to pose additional friction on developers.
		schedule.limits.code_len = 256 * 1024;
		schedule
	};
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type Event = Event;
	type Call = Call;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = frame_support::traits::Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type DeletionQueueDepth = DeletionQueueDepth;
	type DeletionWeightLimit = DeletionWeightLimit;
	type Schedule = Schedule;
	type CallStack = [pallet_contracts::Frame<Self>; 31];
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
	#[cfg(feature = "aura")]
	type OnTimestampSet = Aura;
	#[cfg(feature = "manual-seal")]
	type OnTimestampSet = ();
}

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const ExistentialDeposit: u128 = currency::EXISTENTIAL_DEPOSIT; // 0.0001 ICY
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const TransactionByteFee: Balance = 10 * currency::MILLICENTS;
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction =
        pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = constants::fee::WeightToFee;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
}

impl pallet_sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}

pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.encode()[4..24]));
		}
		None
	}
}

parameter_types! {
	pub const ChainId: u64 = 552;
	pub BlockGasLimit: U256 = U256::from(u32::max_value());
	pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = BaseFee;
	type GasWeightMapping = ();
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressTruncated;
	type WithdrawOrigin = EnsureAddressTruncated;
	type AddressMapping = HashedAddressMapping<BlakeTwo256>;
	type Currency = Balances;
	type Event = Event;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = FrontierPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type OnChargeTransaction = ();
	type FindAuthor = FindAuthorTruncated<Aura>;
}

impl pallet_ethereum::Config for Runtime {
	type Event = Event;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}

frame_support::parameter_types! {
    pub const AssetDeposit: Balance = 500 ;
	pub const AssetAccountDeposit: Balance = 500 ;
	pub const MetadataDepositBase: Balance = 500 ;
    pub const MetaDataDepositPerByte: Balance = 500 ;
    pub const ApprovalDeposit: Balance = 500 ;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetaDataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	type AssetAccountDeposit = ();
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 10 * currency::DOLLARS;
}

impl pallet_vesting::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	// `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
	// highest number of schedules that encodes less than 2^10.
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

// Treasury Pallet
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 3 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const GeneralCouncilMaxMembers: u32 = 100;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = GeneralCouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

// XCM Section

pub const NATIVE_ASSET_ID: u32 = ICZ;

#[derive(codec::Encode, codec::Decode)]
pub enum XCMPMessage<XAccountId, XBalance> {
    /// Transfer tokens to the given account from the Parachain account.
    TransferToken(XAccountId, XBalance),
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type Event = Event;
    type OnSystemEvent = ();
    type SelfParaId = ParachainInfo;
    type DmpMessageHandler = DmpQueue;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type ReservedDmpWeight = ReservedDmpWeight;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type ChannelInfo = ParachainSystem;
	type Event = Event;
	type VersionWrapper = PolkadotXcm;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type ExecuteOverweightOrigin = frame_system::EnsureRoot<AccountId>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}


parameter_types! {
    pub RelayLocation: MultiLocation = MultiLocation::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Polkadot;
    pub RelayCurrency: CurrencyId = DOT;
    pub ParallelNetwork: NetworkId = NetworkId::Named("parallel".into());
    pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = MultiLocation::new(0, X1(Parachain(ParachainInfo::parachain_id().into())));
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

parameter_types! {
    pub const NativeCurrencyId: CurrencyId = NATIVE_ASSET_ID;
    pub GiftAccount: AccountId = PalletId(*b"par/gift").into_account();
}

pub struct GiftConvert;
impl BalanceConversion<Balance, CurrencyId, Balance> for GiftConvert {
    type Error = DispatchError;
    fn to_asset_balance(balance: Balance, asset_id: CurrencyId) -> Result<Balance, Self::Error> {
        let decimal = <Assets as InspectMetadata<AccountId>>::decimals(&asset_id);
        if decimal.is_zero() {
            return Ok(Zero::zero());
        }

        let default_gift_amount = 125 * currency::DOLLARS / 100; // 1.25ICX
        Ok(match asset_id {
            DOT if balance >= 5 * 10_u128.pow(decimal.into()) => default_gift_amount,
            EUSDT | EUSDC if balance >= 300 * 10_u128.pow(decimal.into()) => default_gift_amount,
            _ => Zero::zero(),
        })
    }
}

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = MultiCurrencyAdapter<
	   // Use this currency:
	   CurrencyAdapter,
	   // Use this currency when it is a fungible asset matching the given location or name:
	   IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
	   // Our chain's account ID type (we can't get away without mentioning it explicitly):
	   AccountId,
	   Balance,
	   // Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	   LocationToAccountId,
	   CurrencyIdConvert,
	   NativeCurrencyId,
	   ExistentialDeposit,
	   GiftAccount,
    	GiftConvert,
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

parameter_types! {
    pub DotPerSecond: (AssetId, u128) = (AssetId::Concrete(MultiLocation::parent()), dot_per_second());
    pub ICZPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(ParachainInfo::parachain_id().into()), GeneralKey(b"ICZ".to_vec())),
        ).into(),
        dot_per_second() * 100
    );
    pub ICZPerSecondOfCanonicalLocation: (AssetId, u128) = (
        MultiLocation::new(
            0,
            X1(GeneralKey(b"ICZ".to_vec())),
        ).into(),
        dot_per_second() * 100
    );
	pub ICXPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(ParachainInfo::parachain_id().into()), GeneralKey(b"ICZ".to_vec())),
        ).into(),
        dot_per_second() * 100
    );
    pub ICXPerSecondOfCanonicalLocation: (AssetId, u128) = (
        MultiLocation::new(
            0,
            X1(GeneralKey(b"ICX".to_vec())),
        ).into(),
        dot_per_second() * 100
    );
    pub AusdPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(paras::acala::ID), GeneralKey(paras::acala::AUSD_KEY.to_vec()))
        ).into(),
        dot_per_second() * 30
    );
    pub AcaPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(paras::acala::ID), GeneralKey(paras::acala::ACA_KEY.to_vec()))
        ).into(),
        dot_per_second() * 20
    );
    pub LDOTPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(paras::acala::ID), GeneralKey(paras::acala::LDOT_KEY.to_vec()))
        ).into(),
        dot_per_second()
    );
    pub LCDOTPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(paras::acala::ID), GeneralKey(paras::acala::LCDOT_KEY.to_vec()))
        ).into(),
        dot_per_second()
    );
	pub KARPerSecond: (AssetId, u128) = (
        MultiLocation::new(
            1,
            X2(Parachain(paras::acala::ID), GeneralKey(paras::acala::KAR_KEY.to_vec()))
        ).into(),
        dot_per_second()
    );
}


pub type Barrier = (
    TakeWeightCredit,
    AllowKnownQueryResponses<PolkadotXcm>,
    AllowSubscriptionsFrom<Everything>,
    AllowTopLevelPaidExecutionFrom<Everything>,
);

pub struct ToTreasury;
impl TakeRevenue for ToTreasury {
    fn take_revenue(revenue: MultiAsset) {
        if let MultiAsset {
            id: AssetId::Concrete(id),
            fun: Fungibility::Fungible(amount),
        } = revenue
        {
            if let Some(currency_id) = CurrencyIdConvert::convert(id) {
                let _ = Assets::mint_into(currency_id, &TreasuryAccount::get(), amount);
            }
        }
    }
}


pub type Trader = (
    FixedRateOfFungible<DotPerSecond, ToTreasury>,
    FixedRateOfFungible<ICZPerSecond, ToTreasury>,
    FixedRateOfFungible<ICZPerSecondOfCanonicalLocation, ToTreasury>,
	FixedRateOfFungible<ICXPerSecond, ToTreasury>,
    FixedRateOfFungible<ICXPerSecondOfCanonicalLocation, ToTreasury>,
    FixedRateOfFungible<AusdPerSecond, ToTreasury>,
    FixedRateOfFungible<AcaPerSecond, ToTreasury>,
    FixedRateOfFungible<LDOTPerSecond, ToTreasury>,
    FixedRateOfFungible<LCDOTPerSecond, ToTreasury>,
	FixedRateOfFungible<KARPerSecond, ToTreasury>,
    // Foreign Assets registered in AssetRegistry
    // TODO: replace all above except local reserved asset later
    FirstAssetTrader<AssetType, AssetRegistry, XcmFeesToAccount>,
);

parameter_types! {
    pub CheckingAccount: AccountId = PolkadotXcm::check_account();
}

/// The non-reserve fungible transactor type
/// It will use pallet-assets, and the Id will be matched against AsAssetType
pub type ForeignFungiblesTransactor = FungiblesAdapter<
    // Use this fungibles implementation:
    Assets,
    // Use this currency when it is a fungible asset matching the given location or name:
    (
        ConvertedConcreteAssetId<
            CurrencyId,
            Balance,
            AsAssetType<CurrencyId, AssetType, AssetRegistry>,
            JustTry,
        >,
    ),
    // Do a simple punn to convert an AccountId20 MultiLocation into a native chain account ID:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We dont allow teleports.
    Nothing,
    // We dont track any teleports
    CheckingAccount,
>;

/// How to withdraw and deposit an asset, try LocalAssetTransactor first
/// and if AssetNotFound then with ForeignFungiblesTransactor as fallback
pub type AssetTransactors = (LocalAssetTransactor, ForeignFungiblesTransactor);

/// This is the struct that will handle the revenue from xcm fees
/// We do not burn anything because we want to mimic exactly what
/// the sovereign account has
pub type XcmFeesToAccount = pallet_traits::xcm::XcmFeesToAccount<
    Assets,
    (
        ConvertedConcreteAssetId<
            CurrencyId,
            Balance,
            AsAssetType<CurrencyId, AssetType, AssetRegistry>,
            JustTry,
        >,
    ),
    AccountId,
    TreasuryAccount,
>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type Call = Call;
    type XcmSender = XcmRouter;
    // How to withdraw and deposit an asset.
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;
    // Teleporting is disabled.
    type IsTeleporter = ();
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
    type Trader = Trader;
    type ResponseHandler = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetClaims = PolkadotXcm;
}


impl orml_asset_registry::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type AssetId = CurrencyId;
    type AssetType = AssetType;
    type UpdateOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
    type WeightInfo = ();
}

impl pallet_currency_adapter::Config for Runtime {
    type Assets = Assets;
    type Balances = Balances;
    type GetNativeCurrencyId = NativeCurrencyId;
    type LockOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
}

/// Local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);


impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type LocationInverter = LocationInverter<Ancestry>;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmReserveTransferFilter = Everything;
	type XcmRouter = XcmRouter;
	type XcmTeleportFilter = Nothing;
	type Origin = Origin;
	type Call = Call;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}


impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}


impl orml_xcm::Config for Runtime {
    type Event = Event;
    type SovereignOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
    fn convert(id: CurrencyId) -> Option<MultiLocation> {
        match id {
            DOT => Some(MultiLocation::parent()),
            SDOT => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(ParachainInfo::parachain_id().into()),
                    GeneralKey(b"sDOT".to_vec()),
                ),
            )),
            ICZ => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(ParachainInfo::parachain_id().into()),
                    GeneralKey(b"ICZ".to_vec()),
                ),
            )),
            ACA => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(paras::acala::ID),
                    GeneralKey(paras::acala::ACA_KEY.to_vec()),
                ),
            )),
            AUSD => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(paras::acala::ID),
                    GeneralKey(paras::acala::AUSD_KEY.to_vec()),
                ),
            )),
            LDOT => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(paras::acala::ID),
                    GeneralKey(paras::acala::LDOT_KEY.to_vec()),
                ),
            )),
            LC_DOT => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(paras::acala::ID),
                    GeneralKey(paras::acala::LCDOT_KEY.to_vec()),
                ),
            )),
			KAR => Some(MultiLocation::new(
                1,
                X2(
                    Parachain(paras::acala::ID),
                    GeneralKey(paras::acala::KAR_KEY.to_vec()),
                ),
            )),
            _ => None,
        }
    }
}

impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(location: MultiLocation) -> Option<CurrencyId> {
        match location {
            MultiLocation {
                parents: 1,
                interior: Here,
            } => Some(DOT),
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == ParachainInfo::parachain_id() && key == b"ICX".to_vec() => {
                Some(ICX)
            }
            MultiLocation {
                parents: 0,
                interior: X1(GeneralKey(key)),
            } if key == b"ICX".to_vec() => Some(ICX),
			MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == ParachainInfo::parachain_id() && key == b"ICZ".to_vec() => {
                Some(ICZ)
            }
            MultiLocation {
                parents: 0,
                interior: X1(GeneralKey(key)),
            } if key == b"ICZ".to_vec() => Some(ICZ),
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == paras::acala::ID.into()
                && key == paras::acala::ACA_KEY.to_vec() =>
            {
                Some(ACA)
            }
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == paras::acala::ID.into()
                && key == paras::acala::AUSD_KEY.to_vec() =>
            {
                Some(AUSD)
            }
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == paras::acala::ID.into()
                && key == paras::acala::LDOT_KEY.to_vec() =>
            {
                Some(LDOT)
            }
            MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == paras::acala::ID.into()
                && key == paras::acala::LCDOT_KEY.to_vec() =>
            {
                Some(LC_DOT)
            }
			MultiLocation {
                parents: 1,
                interior: X2(Parachain(id), GeneralKey(key)),
            } if ParaId::from(id) == paras::acala::ID.into()
                && key == paras::acala::KAR_KEY.to_vec() =>
            {
                Some(KAR)
            }
            _ => None,
        }
    }
}

impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(a: MultiAsset) -> Option<CurrencyId> {
        if let MultiAsset {
            id: AssetId::Concrete(id),
            fun: _,
        } = a
        {
            Self::convert(id)
        } else {
            None
        }
    }
}

parameter_types! {
    pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
    pub const BaseXcmWeight: Weight = 150_000_000;
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsForTransfer: usize = 2;
}

// Min fee required when transferring asset back to reserve sibling chain
// which use another asset(e.g Relaychain's asset) as fee
parameter_type_with_key! {
    pub ParachainMinFee: |location: MultiLocation| -> u128 {
        #[allow(clippy::match_ref_pats)] // false positive
        match (location.parents, location.first_interior()) {
            (1, Some(Parachain(paras::statemint::ID))) => XcmHelper::get_xcm_weight_fee_to_sibling(location.clone()).fee,//default fee should be enough even if not configured
            _ => u128::MAX,
        }
    };
}

impl orml_xtokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdtoMultiLocation<
        CurrencyIdConvert,
        AsAssetType<CurrencyId, AssetType, AssetRegistry>,
    >;
    type AccountIdToMultiLocation = AccountIdToMultiLocation<AccountId>;
    type SelfLocation = SelfLocation;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
    type BaseXcmWeight = BaseXcmWeight;
    type LocationInverter = LocationInverter<Ancestry>;
    type MaxAssetsForTransfer = MaxAssetsForTransfer;
    type MinXcmFee = ParachainMinFee;
    type MultiLocationsFilter = Everything;
    type ReserveProvider = AbsoluteReserveProvider;
}

parameter_types! {
    pub const XcmHelperPalletId: PalletId = PalletId(*b"arc/fees");
    pub const NotifyTimeout: BlockNumber = 100;
	pub RefundLocation: AccountId = Utility::derivative_account_id(ParachainInfo::parachain_id().into_account(), u16::MAX);
}

impl pallet_xcm_helper::Config for Runtime {
    type Event = Event;
    type UpdateOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
    type Assets = Assets;
    type XcmSender = XcmRouter;
    type RelayNetwork = RelayNetwork;
    type PalletId = XcmHelperPalletId;
    type NotifyTimeout = NotifyTimeout;
    type AccountIdToMultiLocation = AccountIdToMultiLocation<AccountId>;
    type RefundLocation = RefundLocation;
    type BlockNumberProvider = frame_system::Pallet<Runtime>;
    type XcmOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
    type RelayCurrency = RelayCurrency;
    type WeightInfo = ();
}


// XCM Section ends

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}


parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 10 * currency::DOLLARS;
	pub const SpendPeriod: BlockNumber = DAYS;
	pub const Burn: Permill = Permill::from_percent(1);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");

	pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
	type Currency = Balances;
	type ApproveOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	type RejectOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 5>;
	type Event = Event;
	type OnSlash = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type PalletId = TreasuryPalletId;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	type MaxApprovals = MaxApprovals;
	type ProposalBondMaximum = ();
}

frame_support::parameter_types! {
	pub BoundDivision: U256 = U256::from(1024);
}

impl pallet_dynamic_fee::Config for Runtime {
	type MinGasPriceBoundDivisor = BoundDivision;
}

frame_support::parameter_types! {
	pub IsActive: bool = true;
	pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
}

pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		Permill::from_parts(500_000)
	}
	fn upper() -> Permill {
		Permill::from_parts(1_000_000)
	}
}

impl pallet_base_fee::Config for Runtime {
	type Event = Event;
	type Threshold = BaseFeeThreshold;
	type IsActive = IsActive;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		ParachainSystem: cumulus_pallet_parachain_system::{
			Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
		},
		Utility: pallet_utility::{Pallet, Call, Event},

		ParachainInfo: parachain_info::{Pallet, Storage, Config},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
		Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
		Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
		Ethereum: pallet_ethereum::{Pallet, Call, Storage, Event, Config, Origin},
		EVM: pallet_evm::{Pallet, Config, Call, Storage, Event<T>},
		DynamicFee: pallet_dynamic_fee::{Pallet, Call, Storage, Config, Inherent},

		// Collator support. The order of these 4 are important and shall not change.
		Authorship: pallet_authorship::{Pallet, Call, Storage},
		CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>},
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
		Aura: pallet_aura::{Pallet, Config<T>},
		AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config},

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>},
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin, Config},
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin},
		DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>},

		XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>},
        OrmlXcm: orml_xcm::{Pallet, Call, Event<T>},
		XcmHelper: pallet_xcm_helper::{Pallet, Call, Storage, Event<T>},
	
		BaseFee: pallet_base_fee::{Pallet, Call, Storage, Config<T>, Event},
		Vesting: pallet_vesting::{Pallet, Call, Storage, Config<T>, Event<T>} = 32,
		Assets: pallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
	    Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
        Treasury: pallet_treasury::{Pallet, Call, Storage, Event<T>, Config},
		AssetRegistry: orml_asset_registry::{Pallet, Call, Storage, Event<T>},
        CurrencyAdapter: pallet_currency_adapter::{Pallet, Call},

		//SimpleInflation: pallet_simple_inflation::{Pallet},
	}
);

pub struct TransactionConverter;

impl fp_rpc::ConvertTransaction<UncheckedExtrinsic> for TransactionConverter {
	fn convert_transaction(&self, transaction: pallet_ethereum::Transaction) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_unsigned(
			pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
		)
	}
}

impl fp_rpc::ConvertTransaction<opaque::UncheckedExtrinsic> for TransactionConverter {
	fn convert_transaction(
		&self,
		transaction: pallet_ethereum::Transaction,
	) -> opaque::UncheckedExtrinsic {
		let extrinsic = UncheckedExtrinsic::new_unsigned(
			pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
		);
		let encoded = extrinsic.encode();
		opaque::UncheckedExtrinsic::decode(&mut &encoded[..])
			.expect("Encoded extrinsic is always valid")
	}
}

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = fp_self_contained::CheckedExtrinsic<AccountId, Call, SignedExtra, H160>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

impl fp_self_contained::SelfContainedCall for Call {
	type SignedInfo = H160;

	fn is_self_contained(&self) -> bool {
		match self {
			Call::Ethereum(call) => call.is_self_contained(),
			_ => false,
		}
	}

	fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
		match self {
			Call::Ethereum(call) => call.check_self_contained(),
			_ => None,
		}
	}

	fn validate_self_contained(&self, info: &Self::SignedInfo) -> Option<TransactionValidity> {
		match self {
			Call::Ethereum(call) => call.validate_self_contained(info),
			_ => None,
		}
	}

	fn pre_dispatch_self_contained(
		&self,
		info: &Self::SignedInfo,
	) -> Option<Result<(), TransactionValidityError>> {
		match self {
			Call::Ethereum(call) => call.pre_dispatch_self_contained(info),
			_ => None,
		}
	}

	fn apply_self_contained(
		self,
		info: Self::SignedInfo,
	) -> Option<sp_runtime::DispatchResultWithInfo<PostDispatchInfoOf<Self>>> {
		match self {
			call @ Call::Ethereum(pallet_ethereum::Call::transact { .. }) => Some(call.dispatch(
				Origin::from(pallet_ethereum::RawOrigin::EthereumTransaction(info)),
			)),
			_ => None,
		}
	}
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().to_vec()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {
		fn chain_id() -> u64 {
			<Runtime as pallet_evm::Config>::ChainId::get()
		}

		fn account_basic(address: H160) -> EVMAccount {
			EVM::account_basic(&address)
		}

		fn gas_price() -> U256 {
			<Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price()
		}

		fn account_code_at(address: H160) -> Vec<u8> {
			EVM::account_codes(address)
		}

		fn author() -> H160 {
			<pallet_evm::Pallet<Runtime>>::find_author()
		}

		fn storage_at(address: H160, index: U256) -> H256 {
			let mut tmp = [0u8; 32];
			index.to_big_endian(&mut tmp);
			EVM::account_storages(address, H256::from_slice(&tmp[..]))
		}

		fn call(
			from: H160,
			to: H160,
			data: Vec<u8>,
			value: U256,
			gas_limit: U256,
			max_fee_per_gas: Option<U256>,
			max_priority_fee_per_gas: Option<U256>,
			nonce: Option<U256>,
			estimate: bool,
			access_list: Option<Vec<(H160, Vec<H256>)>>,
		) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
			let is_transactional = false;
			<Runtime as pallet_evm::Config>::Runner::call(
				from,
				to,
				data,
				value,
				gas_limit.low_u64(),
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list.unwrap_or_default(),
				is_transactional,
				config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
			).map_err(|err| err.into())
		}
		fn create(
			from: H160,
			data: Vec<u8>,
			value: U256,
			gas_limit: U256,
			max_fee_per_gas: Option<U256>,
			max_priority_fee_per_gas: Option<U256>,
			nonce: Option<U256>,
			estimate: bool,
			access_list: Option<Vec<(H160, Vec<H256>)>>,
		) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
			let is_transactional = false;
			<Runtime as pallet_evm::Config>::Runner::create(
				from,
				data,
				value,
				gas_limit.low_u64(),
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list.unwrap_or_default(),
				is_transactional,
				config.as_ref().unwrap_or(<Runtime as pallet_evm::Config>::config()),
			).map_err(|err| err.into())
		}

		fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
			Ethereum::current_transaction_statuses()
		}

		fn current_block() -> Option<pallet_ethereum::Block> {
			Ethereum::current_block()
		}

		fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
			Ethereum::current_receipts()
		}

		fn current_all() -> (
			Option<pallet_ethereum::Block>,
			Option<Vec<pallet_ethereum::Receipt>>,
			Option<Vec<TransactionStatus>>
		) {
			(
				Ethereum::current_block(),
				Ethereum::current_receipts(),
				Ethereum::current_transaction_statuses()
			)
		}

		fn extrinsic_filter(
			xts: Vec<<Block as BlockT>::Extrinsic>,
		) -> Vec<EthereumTransaction> {
			xts.into_iter().filter_map(|xt| match xt.0.function {
				Call::Ethereum(transact { transaction }) => Some(transaction),
				_ => None
			}).collect::<Vec<EthereumTransaction>>()
		}

		fn elasticity() -> Option<Permill> {
			Some(BaseFee::elasticity())
		}
	}

	impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {
		fn convert_transaction(transaction: EthereumTransaction) -> <Block as BlockT>::Extrinsic {
			UncheckedExtrinsic::new_unsigned(
				pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
			)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};
			use frame_system_benchmarking::Pallet as SystemBench;

			impl frame_system_benchmarking::Config for Runtime {}
			let whitelist: Vec<TrackedStorageKey> = vec![];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_evm, EVM);
			add_benchmark!(params, batches, pallet_vesting, Vesting);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pallet_evm, EVM);
			list_benchmark!(list, extra, pallet_vesting, Vesting);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade().unwrap();

			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}

	impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
	for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult<Balance> {
			Contracts::bare_call(origin, dest, value, gas_limit, storage_deposit_limit, input_data, CONTRACTS_DEBUG_OUTPUT)
		}

		fn instantiate(
			origin: AccountId,
			value: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts_primitives::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
		{
			Contracts::bare_instantiate(origin, value, gas_limit, storage_deposit_limit, code, data, salt, CONTRACTS_DEBUG_OUTPUT)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
		) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
		{
			Contracts::bare_upload_code(origin, code, storage_deposit_limit)
		}

		fn get_storage(
			address: AccountId,
			key: [u8; 32],
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(address, key)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
