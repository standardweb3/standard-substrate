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

//! # Subswap Module
//!
//! An automated market maker module extended from the [asset](../asset/Module.html) module.
//!
//! ## Overview
//!
//! The Subswap module provides functionality for management and exchange of fungible asset classes
//! with a fixed supply, including:
//!
//! * Liquidity provider token issuance
//! * Compensation for providing liquidity
//! * Automated liquidity provisioning
//! * Asset exchange
//!
//! To use it in your runtime, you need to implement the subswap [`Trait`](./trait.Trait.html).
//!
//! The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.
//!
//! ### Terminology
//!
//! * **Liquidity provider token:** The creation of a new asset by providing liquidity between two fungible assets. Liquidity provider token act as the share of the pool and gets the profit created from exchange fee.
//! * **Asset exchange:** The process of an account transferring an asset to exchange with other kind of fungible asset.
//! * **Fungible asset:** An asset whose units are interchangeable.
//! * **Non-fungible asset:** An asset for which each unit has unique characteristics.
//!
//! ### Goals
//!
//! The Subswap system in Substrate is designed to make the following possible:
//!
//! * Reward liquidity providers with tokens to receive exchanges fees which is proportional to their contribution.
//! * Swap assets with automated market price equation(e.g. X*Y=K or curve function from Kyber, dodoex, etc).
//! * Issue an fungible asset which can be backed with opening exchange with other assets 
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
use frame_system::ensure_root;
use sp_std::{prelude::*, fmt::Debug};
use frame_system::ensure_signed;
use sp_core::U256;
use codec::{Encode, Decode};
use sp_runtime::{RuntimeDebug, traits::{UniqueSaturatedInto, AccountIdConversion}, ModuleId};
use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use pallet_standard_market as market;
use pallet_standard_oracle as oracle;
use primitives::{AssetId, Balance, Amount};
use orml_traits::{MultiCurrency, MultiCurrencyExtended, MultiReservableCurrency};

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct CDP<Balance: Encode + Decode + Clone + Debug + Eq + PartialEq> {
    /// Percentage of liquidator who liquidate the cdp \[numerator, denominator]
	liquidation_fee: (Balance, Balance),
	/// Maximum collaterization rate \[numerator, denominator]
	max_collateraization_rate: (U256, U256),
	/// Fee paid for stability \[numerator, denominator]
    stability_fee: (Balance, Balance)
}
pub const MTR: AssetId = 1_u32;


/// The module configuration trait.
pub trait Config: frame_system::Config + market::Config + oracle::Config {
	/// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    
    /// The Module account for burning assets
    type SystemModuleId: Get<ModuleId>;

    type VaultModuleId: Get<ModuleId>;

    type Currency: MultiCurrencyExtended<Self::AccountId, CurrencyId = AssetId, Balance = Balance, Amount = Amount>
			+ MultiReservableCurrency<Self::AccountId>;

}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		type Error = Error<T>;

        fn deposit_event() = default;

        #[weight= 0]
        fn generate(
            origin,
            #[compact] request_amount: Balance,
            #[compact] collateral_id: AssetId, 
            #[compact] collateral_amount: Balance) {
            let origin = ensure_signed(origin)?;
            // Get position for the collateral
            let position = Self::position(collateral_id);
            ensure!(position.is_some(), Error::<T>::CollateralNotSupported);
            // Get price from oracles
            let collateral_price = oracle::Module::<T>::price(collateral_id)?;
            let mtr_price = oracle::Module::<T>::price(MTR)?;
            // Get vault from sender and divide cases
            let (total_collateral, total_request) = match Self::vault((origin.clone(), collateral_id)) {
                // vault exists for the sender
                Some(x) => {
                    // Add collateral and mtr amount from existing vault
                    let collateral_total = collateral_amount + x.0;
                    let request_total = request_amount + x.1;  
                    (collateral_total, request_total)
                },
                // vault does not exist for the sender
                None => {
                    (collateral_amount, request_amount)
                }
            };

            let result = Self::is_cdp_valid(&position.unwrap(), collateral_price, total_collateral, mtr_price, total_request);
            // Check whether CDP is valid
            ensure!(result, Error::<T>::InvalidCDP);
            
            // Send collateral to Standard Protocol
            <T as Config>::Currency::transfer(collateral_id, &origin, &Self::sys_account_id(), collateral_amount)?;

            // Update CDP
            <Vault<T>>::mutate((origin.clone(), collateral_id), |vlt|{
                *vlt = Some((total_collateral, total_request));
            });

            // Send mtr to sender
            <T as Config>::Currency::transfer(MTR, &origin, &Self::sys_account_id(), request_amount)?;

            // deposit event
            Self::deposit_event(RawEvent::UpdateVault(origin, collateral_id, total_collateral, request_amount))
        }


        #[weight=0]
        fn liquidate_vault(
            origin,
            account: T::AccountId,
            #[compact] collateral_id: AssetId) {
            let origin = ensure_signed(origin)?;
            let vault = <Vault<T>>::get((account.clone(), collateral_id));
            ensure!(vault.is_some(), Error::<T>::VaultDoesNotExist);
            // check if the vault is still valid
            let position = Self::position(collateral_id);
            ensure!(position.is_some(), Error::<T>::CollateralNotSupported);
            // Get price from oracles
            let collateral_price = oracle::Module::<T>::price(collateral_id)?;
            let mtr_price = oracle::Module::<T>::price(MTR)?;
            let (collateral_amount, request_amount) = vault.unwrap();
            let result = Self::is_cdp_valid(&position.clone().unwrap(), collateral_price, collateral_amount, mtr_price, request_amount);
            // Check whether cdp is invalid
            ensure!(!result, Error::<T>::Unavailable);
            // liquidate the vault
            // Pay liquidation fee to the liquidator
            let liquidation_rate = position.unwrap().liquidation_fee;
            let fee = collateral_amount/liquidation_rate.1*liquidation_rate.0;
            <T as Config>::Currency::transfer(collateral_id, &origin, &Self::account_id(), fee)?;

            let rest = collateral_amount - fee;
            // Check pairs in the market
            let lpt = market::Pairs::get((MTR, collateral_id.clone()));
            ensure!(lpt.is_some(), Error::<T>::MarketDoesNotExist);

            // Send collateral to the market
            let reserves = market::Reserves::get(lpt.unwrap());
            let liquidated = rest+reserves.1;
            market::Module::<T>::_set_reserves(MTR, collateral_id, reserves.0, liquidated, lpt.unwrap());
            
            // destroy the vault
            <Vault<T>>::take((account.clone(), collateral_id.clone()));

            // deposit event
            Self::deposit_event(RawEvent::Liquidate(collateral_id, collateral_amount));
        }

        #[weight=0]
        fn close(
            origin, 
            #[compact] collateral_id: AssetId) {
            let origin = ensure_signed(origin)?;
            let vault = Vault::<T>::get((origin.clone(), collateral_id));
            ensure!(vault.is_some(), Error::<T>::VaultDoesNotExist);
            // check if the vault is still valid
            let position = Self::position(collateral_id);
            ensure!(position.is_some(), Error::<T>::CollateralNotSupported);
            // Get price from oracles
            let collateral_price = oracle::Module::<T>::price(collateral_id)?;
            let mtr_price = oracle::Module::<T>::price(MTR)?;
            let (collateral_amount, request_amount) = vault.unwrap();
            let result = Self::is_cdp_valid(&position.clone().unwrap(), collateral_price, collateral_amount, mtr_price, request_amount);
            // Check whether cdp is valid and safe from liquidation.
            ensure!(result, Error::<T>::AddMoreCollateral);
            // close the vault
            
            // Pay stability fee with collateral to the Standard treasury
            let stability_rate = position.unwrap().stability_fee;
            let fee = collateral_amount/stability_rate.1*stability_rate.0;
            <T as Config>::Currency::transfer(collateral_id, &Self::account_id(), &Self::sys_account_id(), fee)?;

            let rest = collateral_amount - fee;
             
            // Give back the collateral
            let _ = <T as Config>::Currency::transfer(collateral_id, &Self::sys_account_id(), &origin, rest);

            // deposit event
            Self::deposit_event(RawEvent::CloseVault(collateral_id, collateral_amount, request_amount));
            
        }

        #[weight=0]
        fn set_position(
            origin,
            collateral_id: AssetId,
            liqudation_rate: (Balance, Balance),
            max_collateraization_rate: (U256, U256),
            stability_fee: (Balance, Balance)
        ) {
            ensure_root(origin)?;

            Positions::insert(collateral_id, CDP{
                liquidation_fee: liqudation_rate,
                max_collateraization_rate,
                stability_fee
            });

            // deposit event
            Self::deposit_event(RawEvent::SetPosition(collateral_id, liqudation_rate.0, liqudation_rate.1, max_collateraization_rate.0, max_collateraization_rate.1, stability_fee.0, stability_fee.1));
        }
	}
}

decl_event! {
    pub enum Event<T> where
        <T as frame_system::Config>::AccountId,
		Balance = Balance,
		AssetId = AssetId,
	{
        /// A vault is created with the collateral. \[who, collateral, collateral_amount, meter_amount]
		UpdateVault(AccountId, AssetId, Balance, Balance), 
		/// A vault is liquidated \[collateral, collateral_amount]
		Liquidate(AssetId, Balance),
		/// Close vault by paying back meter. \[collateral, collateral_amount, paid_meter_amount]
        CloseVault(AssetId, Balance, Balance),
        /// Set position for collateral. \[collateral, liquidation_fee[numerator/denominator], max_collateraization_rate[numerator/denominator], stability_fee[numerator/denominator]]
        SetPosition(AssetId, Balance, Balance, U256, U256, Balance, Balance),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
        /// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// No value
		NoneValue,
        /// Collateral is not supported
        CollateralNotSupported,
        /// Invalid CDP
        InvalidCDP,
        /// Need more collateral to close the vault,
        AddMoreCollateral,
        /// Unavailable to Liquidate
        Unavailable,
        /// Vault does not exist
        VaultDoesNotExist,
        /// Market does not exist
        MarketDoesNotExist
	}
}

decl_storage! {
	trait Store for Module<T: Config> as Vault {
        // Vault to keep the number of collatral amount and meter amount. \[collateral_amount, meter_amount]
        pub Vault get(fn vault): map hasher(blake2_128_concat) (T::AccountId, AssetId) => Option<(Balance, Balance)>;
        pub Positions get(fn position): map hasher(blake2_128_concat) AssetId => Option<CDP<Balance>>;
        pub CirculatingSupply get(fn circulating_supply): Balance;
	}
}

impl<T: Config> Module<T> {

    // Module account id
    pub fn account_id() -> T::AccountId  {
		<T as Config>::VaultModuleId::get().into_account()
    }
    
    // System account id
    pub fn sys_account_id() -> T::AccountId  {
        <T as Config>::SystemModuleId::get().into_account()
    }
    

    fn is_cdp_valid(position: &CDP<Balance>, collateral_price: Balance, collateral_amount: Balance, request_price: Balance, request_amount: Balance) -> bool {
        let collateral_price_256 = Self::to_u256(collateral_price);
        let mtr_price_256 = Self::to_u256(request_price);
        let total_collateral_256 = Self::to_u256(collateral_amount);
        let collateral = collateral_price_256.checked_mul(total_collateral_256).expect("Multiplication overflow");
        let total_request_256 = Self::to_u256(request_amount);
        let request = mtr_price_256.checked_mul(total_request_256).expect("Multiplication overflow");
        let determinant = collateral.checked_div(position.max_collateraization_rate.1).expect("divided by zero").checked_mul(position.max_collateraization_rate.0).unwrap_or(U256::from(0));
        request < determinant
    }
    
    pub fn to_u256(value: Balance) -> U256 {
        U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(value))
    }

}



