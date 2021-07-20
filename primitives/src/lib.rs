#![cfg_attr(not(feature = "std"), no_std)]

/// Balance for and account
pub type Balance = u128;

/// Index for identifying an asset
pub type AssetId = u32;

/// Amount to send a currency
pub type Amount = i128;

/// Index for identifying currency
pub type CurrencyId = u32;

/// Counter for the number of eras that have passed.
pub type EraIndex = u64;

/// Index for oracle to provide information
pub type SlotIndex = u32;

pub const CORE_ASSET_ID: AssetId = 0;
