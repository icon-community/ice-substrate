#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	
	fn set_airdrop_server_account() -> Weight;
	fn dispatch_user_claim() -> Weight;
	fn dispatch_exchange_claim() -> Weight;
	fn update_airdrop_state() -> Weight;
	fn change_merkle_root()->Weight;
}

/// Weight functions for `pallet_airdrop`.
pub struct AirDropWeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for AirDropWeightInfo<T>{
	// Storage: Airdrop ServerAccount (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	fn set_airdrop_server_account() -> Weight {
		(20_566_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Airdrop AirdropChainState (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	fn update_airdrop_state() -> Weight {
		(20_384_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Airdrop AirdropChainState (r:1 w:0)
	// Storage: Airdrop MerkleRoot (r:1 w:0)
	// Storage: Airdrop IconSnapshotMap (r:1 w:1)
	// Storage: Airdrop IceIconMap (r:1 w:1)
	// Storage: Airdrop CreditorAccount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn dispatch_user_claim() -> Weight {
		(246_184_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Airdrop AirdropChainState (r:1 w:0)
	// Storage: Airdrop ExchangeAccountsMap (r:1 w:0)
	// Storage: Airdrop MerkleRoot (r:1 w:0)
	// Storage: Airdrop CreditorAccount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Airdrop IconSnapshotMap (r:1 w:1)
	// Storage: Airdrop IceIconMap (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn dispatch_exchange_claim() -> Weight {
		(128_584_000 as Weight)
			// Standard Error: 156_000
			.saturating_add(392_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Airdrop MerkleRoot (r:1 w:1)
	// Storage: System Number (r:1 w:0)
	// Storage: System ExecutionPhase (r:1 w:0)
	// Storage: System EventCount (r:1 w:1)
	// Storage: System Events (r:1 w:1)
	fn change_merkle_root() -> Weight {
		(26_243_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

