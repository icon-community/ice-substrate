use crate::{self as pallet_airdrop, types};
use core::marker::PhantomData;

use frame_support::{parameter_types, traits::ConstU32};
use frame_system as system;
use pallet_balances;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type Index = u64;
type BlockNumber = u64;

pub struct TestValidator<T>(PhantomData<T>);

impl types::MerkelProofValidator<Test> for TestValidator<Test> {
	fn validate(
		_root_hash: types::MerkleHash,
		_leaf_hash: types::MerkleHash,
		_proofs: types::MerkleProofs<Test>,
	) -> bool {
		return true;
	}
}

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		AirdropModule: pallet_airdrop::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Vesting: pallet_vesting::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u16 = 2208;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
	pub const VestingMinTransfer: Balance = 1000;
}

impl pallet_airdrop::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type AirdropWeightInfo = pallet_airdrop::weights::AirDropWeightInfo<Test>;
	type BalanceTypeConversion = sp_runtime::traits::ConvertInto;
	type MerkelProofValidator = TestValidator<Test>;
	type MaxProofSize = ConstU32<10>;

	const VESTING_TERMS: types::VestingTerms = {
		types::VestingTerms {
			defi_instant_percentage: 40,
			non_defi_instant_percentage: 30,
			vesting_period: 5_256_000,
		}
	};
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

impl pallet_vesting::Config for Test {
	type Event = Event;
	type Currency = <Test as pallet_airdrop::Config>::Currency;
	type BlockNumberToBalance = sp_runtime::traits::ConvertInto;
	type MinVestedTransfer = VestingMinTransfer;
	type WeightInfo = ();
	const MAX_VESTING_SCHEDULES: u32 = 10;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
