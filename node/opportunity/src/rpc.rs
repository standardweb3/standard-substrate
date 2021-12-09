// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of node-specific RPC methods.
//!
//! Since `substrate` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `sc-rpc` crate.
//! It means that `client/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what FRAME pallets
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding FRAME RPC definitions.

#![warn(missing_docs)]

use std::sync::Arc;

use fc_rpc::{
	EthApi, EthApiServer, EthBlockDataCache, EthFilterApi, EthFilterApiServer, EthPubSubApi,
	EthPubSubApiServer, HexEncodedIdProvider, NetApi, NetApiServer, OverrideHandle,
	RuntimeApiStorageOverride, SchemaV1Override, SchemaV2Override, StorageOverride, Web3Api,
	Web3ApiServer,
};
use fc_rpc_core::types::FilterPool;
use jsonrpc_pubsub::manager::SubscriptionManager;
use opportunity_runtime::{AccountId, BlockNumber, Hash, Index};
use pallet_ethereum::EthereumStorageSchema;
use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
use primitives::{Balance, Block};
use sc_client_api::{AuxStore, BlockchainEvents, StorageProvider};
use sc_consensus_babe::{Config, Epoch};
use sc_consensus_babe_rpc::BabeRpcHandler;
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sc_network::NetworkService;
use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use sc_service::Error as ServiceError;
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::SyncCryptoStorePtr;
use std::collections::BTreeMap;
use substrate_frame_rpc_system::{FullSystem, SystemApi};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
/// RPC result.
pub type RpcResult = Result<RpcExtension, ServiceError>;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B, T, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// EthFilterApi pool.
	pub filter_pool: FilterPool,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// The Node authority flag
	pub is_authority: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Ethereum transaction conversion helper.
	pub transaction_converter: T,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B, T, A>(
	deps: FullDeps<C, P, SC, B, T, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
) -> RpcResult
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
	C: sc_client_api::client::BlockchainEvents<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	SC: SelectChain<Block> + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
	T: fp_rpc::ConvertTransaction<sp_runtime::OpaqueExtrinsic> + Sync + Send + 'static,
	A: ChainApi<Block = Block> + 'static,
{
	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		graph,
		select_chain,
		chain_spec,
		deny_unsafe,
		babe,
		grandpa,
		network,
		filter_pool,
		backend,
		transaction_converter,
		is_authority,
	} = deps;

	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(
		client.clone(),
		pool.clone(),
		deny_unsafe,
	)));
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));

	io.extend_with(sc_consensus_babe_rpc::BabeApi::to_delegate(BabeRpcHandler::new(
		client.clone(),
		shared_epoch_changes.clone(),
		keystore,
		babe_config,
		select_chain,
		deny_unsafe,
	)));

	io.extend_with(sc_finality_grandpa_rpc::GrandpaApi::to_delegate(GrandpaRpcHandler::new(
		shared_authority_set.clone(),
		shared_voter_state,
		justification_stream,
		subscription_executor,
		finality_provider,
	)));

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
		backend.clone(),
		is_authority,
		max_past_logs,
		block_data_cache.clone(),
	)));

	io.extend_with(EthFilterApiServer::to_delegate(EthFilterApi::new(
		client.clone(),
		backend,
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

	Ok(io)
}
