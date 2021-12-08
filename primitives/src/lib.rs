#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{generic, traits::BlakeTwo256, OpaqueExtrinsic};

/// An index to a block.
pub type BlockNumber = u32;
/// Balance for and account
pub type Balance = u128;
/// Index for identifying an asset
pub type AssetId = u32;
/// Amount to send a currency
pub type Amount = i128;
/// Index for identifying currency
pub type CurrencyId = u32;
/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;
/// Counter for the number of eras that have passed.
pub type EraIndex = u64;
/// Index for oracle to provide information
pub type SocketIndex = u32;
/// Primary asset ID to use
pub const CORE_ASSET_ID: AssetId = 0;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
