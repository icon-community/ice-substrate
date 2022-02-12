
use frame_support::{
	parameter_types,
};
use frame_system as system;

use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Identity, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
		Vesting: pallet_vesting::{Pallet, Call, Storage, Config<T>, Event<T>}
	}
);
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Test {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type OnSetCode = ();

	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

/// Balance of an account.
pub type Balance = u64;

parameter_types! {
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
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

frame_support::parameter_types! {
	pub const AssetDeposit: Balance = 500 ;
	pub const AssetAccountDeposit: Balance = 500 ;
	pub const MetadataDepositBase: Balance = 500 ;
	pub const MetaDataDepositPerByte: Balance = 500 ;
	pub const ApprovalDeposit: Balance = 500 ;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetaDataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const MinVestedTransfer: u64 = 256 * 2;
	pub const ExistentialDeposit: u64 = 256;
}
impl pallet_vesting::Config for Test {
	type BlockNumberToBalance = Identity;
	type Currency = Balances;
	type Event = Event;
	const MAX_VESTING_SCHEDULES: u32 = 3;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = ();
}

pub struct ExtBuilder {
	existential_deposit: u64,
	vesting_genesis_config: Option<Vec<(u64, u64, u64, u64)>>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1, vesting_genesis_config: None }
	}
}

impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}

	pub fn vesting_genesis_config(mut self, config: Vec<(u64, u64, u64, u64)>) -> Self {
		self.vesting_genesis_config = Some(config);
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		
		// EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
		
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
				(1, 10 * self.existential_deposit),
				(2, 20 * self.existential_deposit),
				(3, 30 * self.existential_deposit),
				(4, 40 * self.existential_deposit),
				(12, 10 * self.existential_deposit),
				(13, 9999 * self.existential_deposit),
			],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let vesting = if let Some(vesting_config) = self.vesting_genesis_config {
			vesting_config
		} else {
			vec![
				(1, 0, 10, 5 * self.existential_deposit),
				(2, 10, 20, 0),
				(12, 10, 20, 5 * self.existential_deposit),
			]
		};

		pallet_vesting::GenesisConfig::<Test> { vesting }
			.assimilate_storage(&mut t)
			.unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
	
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}


