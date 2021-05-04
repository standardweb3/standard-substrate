#![cfg_attr(not(feature = "std"), no_std)]

pub type Balance = u128;
pub type AssetId = u32;
pub type Amount = i128;
pub type CurrencyId = u32;

pub const CORE_ASSET_ID: AssetId = 0;
