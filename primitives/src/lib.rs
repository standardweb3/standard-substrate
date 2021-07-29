#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{generic, traits::{BlakeTwo256}, OpaqueExtrinsic};

pub type BlockNumber = u32;
pub type Balance = u128;
pub type AssetId = u32;
pub type Amount = i128;
pub type CurrencyId = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

pub const CORE_ASSET_ID: AssetId = 0;
