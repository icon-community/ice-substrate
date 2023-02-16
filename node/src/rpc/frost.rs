//! Ice RPCs implementation.

use super::FullDeps;
use crate::primitives::*;
use fc_rpc::{
	Eth, EthApiServer, EthBlockDataCacheTask, EthFilter, EthFilterApiServer, EthPubSub,
	EthPubSubApiServer, Net, NetApiServer, OverrideHandle, RuntimeApiStorageOverride,
	SchemaV1Override, SchemaV2Override, SchemaV3Override, StorageOverride, Web3, Web3ApiServer,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use fp_storage::EthereumStorageSchema;
use jsonrpsee::RpcModule;
use pallet_rmrk_rpc::{Rmrk, RmrkApiServer};
use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
use sc_client_api::{AuxStore, Backend, BlockchainEvents, StateBackend, StorageProvider};
use sc_network::NetworkService;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_runtime::traits::BlakeTwo256;
use std::collections::BTreeMap;
use std::sync::Arc;
use substrate_frame_rpc_system::{System, SystemApiServer};

/// Instantiate all RPC extensions.
pub fn create_full<C, P, BE, A>(
	deps: FullDeps<C, P, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ StorageProvider<Block, BE>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	BE::Blockchain: BlockchainBackend<Block>,
	A: ChainApi<Block = Block> + 'static,
{
	let mut io = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		graph,
		network,
		deny_unsafe,
		is_authority,
		frontier_backend,
		filter_pool,
		fee_history_limit,
		fee_history_cache,
		overrides,
		block_data_cache,
	} = deps;

	io.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	io.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph,
			Some(arctic_runtime::TransactionConverter),
			network.clone(),
			Default::default(),
			overrides.clone(),
			frontier_backend.clone(),
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_limit,
			10,
		)
		.into_rpc(),
	)?;

	let max_past_logs: u32 = 10_000;
	let max_stored_filters: usize = 500;
	io.merge(
		EthFilter::new(
			client.clone(),
			frontier_backend,
			filter_pool,
			max_stored_filters,
			max_past_logs,
			block_data_cache,
		)
		.into_rpc(),
	)?;

	io.merge(Net::new(client.clone(), network.clone(), true).into_rpc())?;

	io.merge(Web3::new(client.clone()).into_rpc())?;

	io.merge(
		EthPubSub::new(pool, client, network, subscription_task_executor, overrides).into_rpc(),
	)?;
	// need to import types from runtime which is not ideal.
	// io.merge(Rmrk::new(client.clone()).into_rpc())?;

	Ok(io)
}
