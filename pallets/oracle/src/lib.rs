// This file is part of Substrate.

// Copyright (C) Hyungsuk Kang
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

//! # Oracle Module
//!
//! An oracle provider module with ledger to reward and slashes.
//!
//! ## Overview
//!
//! The Oracle module provides functionality for management and exchange of fungible asset classes
//! with a fixed supply, including:
//!
//! * Price aggregator from internal/external provider of Standard ecosystem
//!
//! To use it in your runtime, you need to implement the subswap [`Trait`](./trait.Trait.html).
//!
//! The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.
//!
//! ### Terminology
//!
//! * **Oracle:** A verified worker to bring external information to blockchain.
//!
//! ### Goals
//!
//! The Oracle module in Standard protocol is designed to make the following possible:
//!
//! * Reward oracle providers with tokens to receive oracle fees which is proportional to their contribution with precision.
//! * Provide price information for other digital or fiat assets so that Standard system can synthetically generate its digital assets including stablecoin, stocks, etc.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `issue` - Issues the total supply of a new fungible asset to the account of the caller of the function.
//! * `mint` - Mints the asset to the account in the argument with the requested amount from the caller. Caller must be the creator of the asset.
//! * `burn` - Burns the asset from the caller by the amount in the argument 
//! * `transfer` - Transfers an `amount` of units of fungible asset `id` from the balance of
//! the function caller's account (`origin`) to a `target` account.
//! * `destroy` - Destroys the entire holding of a fungible asset `id` associated with the account
//! that called the function.
//! * `mint_liquidity` - Mints liquidity token by adding deposits to a certain pair for exchange. The assets must have different identifier.
//! * `burn_liquidity` - Burns liquidity token for a pair and receives each asset in the pair.  
//! * `swap` - Swaps from one asset to the another, paying 0.3% fee to the liquidity providers.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//!
//! ### Public Functions
//!
//! * `balance` - Get the balance of the account with the asset id
//! * `total_supply` - Get the total supply of an asset.
//! * `mint_from_system` - Mint asset from the system to an account, increasing total supply.
//! * `burn_from_system` - Burn asset from the system to an account, decreasing total supply.
//! * `transfer_from_system - Transfer asset from an account to the system with no change in total supply.
//! * `transfer_to_system - Transfer asset from system to the user with no chang in total supply.
//! * `issue_from_system` - Issue asset from system 
//! * `swap` - Swap one asset to another asset
//! 
//! Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available functions.
//!
//! ## Usage
//!
//! The following example shows how to use the Subswap module in your runtime by exposing public functions to:
//!
//! * Issue and manage a new fungible asset.
//! * Query the fungible asset holding balance of an account.
//! * Query the total supply of a fungible asset that has been issued.
//! * Manage existing asset for other business logic
//!
//! ### Prerequisites
//!
//! Import the Subswap module and types and derive your runtime's configuration traits from the Assets module trait.
//!
//! ### Simple Code Snippet
//!
//! ```rust,ignore
//! use subswap;
//! use pallet_balances as balances;
//! use frame_support::{decl_module, dispatch, ensure};
//! use frame_system::ensure_signed;
//!
//! pub trait Trait: subswap::Trait + balances::Trait {
//! 
//!  }
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! 		pub fn trade(origin, token0: T::AssetId, amount0: <T as balances::Trait>::Balance, token1: T::AssetId) -> dispatch::DispatchResult {
//! 			let sender = ensure_signed(origin).map_err(|e| e.as_str())?;
//!
//!             let amount_out = subswap::Module<T>::swap(&token0, &amount0, &token1); 
//! 			
//! 			Self::deposit_event(RawEvent::Trade(token0, amount0, token1, amount_out));
//! 			Ok(())
//! 		}
//! 	}
//! }
//! ```
//!
//! ## Assumptions
//!
//! Below are assumptions that must be held when using this module.  If any of
//! them are violated, the behavior of this module is undefined.
//!
//! * The total count of assets should be less than
//!   `Trait::AssetId::max_value()`.
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure};
use frame_system::{ensure_signed };
use sp_runtime::{ DispatchResult, DispatchError};
use sp_std::prelude::*;
use primitives::{Balance, AssetId};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// Uniquely identify a request's specification understood by an Operator
pub type SpecIndex = Vec<u8>;
// Uniquely identify a request for a considered Operator
pub type RequestIdentifier = u64;
// The version of the serialized data format
pub type DataVersion = u64;

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

        fn deposit_event() = default;
        

        // REVIEW: Use `///` instead of `//` to make these doc comments that are part of the crate documentation.
		// Register a new Operator.
		// Fails with `OperatorAlreadyRegistered` if this Operator (identified by `origin`) has already been registered.
		#[weight = 10_000]
		pub fn register_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			ensure!(!<Operators<T>>::get(&who), Error::<T>::OperatorAlreadyRegistered);

			Operators::<T>::insert(&who, true);

			Self::deposit_event(RawEvent::OperatorRegistered(who));

			Ok(())
		}

		// Unregisters an existing Operator
		// TODO check weight
		#[weight = 10_000]
		pub fn unregister_operator(origin) -> DispatchResult {
			let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;

			if Operators::<T>::take(who.clone()) {
				Self::deposit_event(RawEvent::OperatorUnregistered(who));
				Ok(())
			} else {
				Err(Error::<T>::UnknownOperator.into())
			}
        }
        
        #[weight = 0]
        fn report(origin, _id: AssetId, _price: Balance) {
            let who : <T as frame_system::Config>::AccountId = ensure_signed(origin)?;
			ensure!(Operators::<T>::contains_key(who), Error::<T>::WrongOperator);
			
        }

	}
}

decl_event! {
	pub enum Event<T> where
	    <T as frame_system::Config>::AccountId,
		Balance = Balance,
	{
		// A request has been accepted. Corresponding fee paiement is reserved
		OracleRequest(AccountId, SpecIndex, RequestIdentifier, AccountId, DataVersion, Vec<u8>, Vec<u8>, Balance),

		// A request has been answered. Corresponding fee paiement is transfered
		OracleAnswer(AccountId, RequestIdentifier, AccountId, Vec<u8>, Balance),

		// A new operator has been registered
		OperatorRegistered(AccountId),

		// An existing operator has been unregistered
		OperatorUnregistered(AccountId),

		// A request didn't receive any result in time
		KillRequest(RequestIdentifier),
	}
}


decl_error! {
	pub enum Error for Module<T: Config>  {
        /// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
        StorageOverflow,
        // Manipulating an unknown operator
		UnknownOperator,
		// Manipulating an unknown request
		UnknownRequest,
		// Not the expected operator
		WrongOperator,
		// An operator is already registered.
		OperatorAlreadyRegistered,
		// Callback cannot be deserialized
		UnknownCallback,
		// Fee provided does not match minimum required fee
		InsufficientFee,
		// Price does not exist
		PriceDoesNotExist,
	}
}

decl_storage! {
	trait Store for Module<T: Config> as Oracle {
		// the result of the oracle call
        pub Result get(fn get_result): i128;
        
        // A set of all registered Operator
        pub Operators get(fn operator): map hasher(blake2_128_concat) <T as frame_system::Config>::AccountId => bool;
        
        pub Prices get(fn asset_price): map hasher(blake2_128_concat) AssetId =>  Option<Balance>;

	} add_extra_genesis {
		config(oracles):
			Vec<<T as frame_system::Config>::AccountId>;
		build(|config: &GenesisConfig<T>| {
			for oracle in &config.oracles {
				Operators::<T>::insert(oracle, true);
			}
		});
	}
}

// The main implementation block for the module.
impl<T: Config> Module<T> {
	pub fn price(id: AssetId) -> sp_std::result::Result<Balance, DispatchError> {
		match Self::asset_price(id) {
			Some(x) => {
				return Ok(x)
			},
			None => {
				return Err(DispatchError::from(crate::Error::<T>::PriceDoesNotExist).into());
			}
		}
		
    }
}

