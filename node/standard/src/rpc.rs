//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use primitives::{AccountId, Balance, Block, Hash, Index as Nonce};

use fc_rpc_core::types::FilterPool;
use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
use sc_client_api::{AuxStore, Backend, BlockchainEvents, StateBackend, StorageProvider};
use sc_network::NetworkService;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_service::Error as ServiceError;
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_runtime::traits::BlakeTwo256;
use substrate_frame_rpc_system::{FullSystem, SystemApi};

use fc_rpc::{
	EthApi, EthApiServer, EthBlockDataCache, EthFilterApi, EthFilterApiServer, EthPubSubApi,
	EthPubSubApiServer, HexEncodedIdProvider, NetApi, NetApiServer, OverrideHandle,
	RuntimeApiStorageOverride, SchemaV1Override, SchemaV2Override, StorageOverride, Web3Api,
	Web3ApiServer,
};
use jsonrpc_pubsub::manager::SubscriptionManager;
use pallet_ethereum::EthereumStorageSchema;
use std::collections::BTreeMap;

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
/// RPC result.
pub type RpcResult = Result<RpcExtension, ServiceError>;

/// Full client dependencies
pub struct FullDeps<C, P, T, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// The Node authority flag
	pub is_authority: bool,
	/// Frontier Backend.
	pub frontier_backend: Arc<fc_db::Backend<Block>>,
	/// Ethereum transaction conversion helper.
	pub transaction_converter: T,
	/// EthFilterApi pool.
	pub filter_pool: FilterPool,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, T, B, A>(
	deps: FullDeps<C, P, T, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
) -> RpcExtension
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ StorageProvider<Block, B>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	T: fp_rpc::ConvertTransaction<sp_runtime::OpaqueExtrinsic> + Sync + Send + 'static,
	B: Backend<Block> + 'static,
	B::State: StateBackend<BlakeTwo256>,
	B::Blockchain: BlockchainBackend<Block>,
	A: ChainApi<Block = Block> + 'static,
{
	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		deny_unsafe,
		graph,
		network,
		is_authority,
		frontier_backend,
		transaction_converter,
		filter_pool,
	} = deps;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(
		client.clone(),
		pool.clone(),
		deny_unsafe,
	)));
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));

	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V2,
		Box::new(SchemaV2Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	let overrides = Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	});

	let max_past_logs: u32 = 10_000;
	let max_stored_filters: usize = 500;
	let block_data_cache = Arc::new(EthBlockDataCache::new(50, 50));

	io.extend_with(EthApiServer::to_delegate(EthApi::new(
		client.clone(),
		pool.clone(),
		graph,
		transaction_converter,
		network.clone(),
		Default::default(),
		overrides.clone(),
		frontier_backend.clone(),
		is_authority,
		max_past_logs,
		block_data_cache.clone(),
	)));

	io.extend_with(EthFilterApiServer::to_delegate(EthFilterApi::new(
		client.clone(),
		frontier_backend,
		filter_pool,
		max_stored_filters,
		overrides.clone(),
		max_past_logs,
		block_data_cache.clone(),
	)));

	io.extend_with(NetApiServer::to_delegate(NetApi::new(client.clone(), network.clone(), true)));

	io.extend_with(Web3ApiServer::to_delegate(Web3Api::new(client.clone())));

	io.extend_with(EthPubSubApiServer::to_delegate(EthPubSubApi::new(
		pool,
		client.clone(),
		network,
		SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
			HexEncodedIdProvider::default(),
			Arc::new(subscription_task_executor),
		),
		overrides,
	)));

	io
}
