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

//! # Standard Market Module
//!
//! An automated market maker module extended from the [asset](../asset/Module.html) module.
//!
//! ## Overview
//!
//! The Standard Market module provides functionality for management and exchange of fungible asset classes
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
//! The Standard system in Substrate is designed to make the following possible:
//!
//! * Reward liquidity providers with tokens to receive exchanges fees which is proportional to their contribution.
//! * Swap assets with automated market price equation(e.g. X*Y=K or curve function from Kyber, dodoex, etc).
//! * Issue an fungible asset which can be backed with opening exchange with other assets 
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! that called the function.
//! * `mint_liquidity` - Mints liquidity token by adding deposits to a certain pair for exchange. The assets must have different identifier.
//! * `burn_liquidity` - Burns liquidity token for a pair and receives each asset in the pair.  
//! * `swap` - Swaps from one asset to the another, paying 0.3% fee to the liquidity providers.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//!
//! ### Public Functions
//!
//! 
//! Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available functions.
//!
//! ## Usage
//!
//! The following example shows how to use the Subswap module in your runtime by exposing public functions to:
//!
//! ### Prerequisites
//!
//! Import the Subswap module and types and derive your runtime's configuration traits from the Assets module trait.
//!
//! ### Simple Code Snippet
//!
//! ```rust,ignore
//! ```
//!
//! ## Assumptions
//!
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`StandardToken`](../token/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{ decl_module, decl_event, decl_storage, decl_error, ensure, dispatch, traits::{Get}};
use sp_runtime::traits::{ Zero, };
use frame_system::ensure_signed;
use sp_core::U256;
use sp_runtime::{FixedU128, ModuleId, traits::{UniqueSaturatedInto, UniqueSaturatedFrom}};
use sp_runtime::traits::{  AccountIdConversion};
use primitives::{AssetId, Balance, Amount};
use orml_traits::{MultiCurrency, MultiCurrencyExtended, MultiReservableCurrency};
use pallet_asset_registry;
//use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
mod math;


/// The module configuration trait.
pub trait Config: frame_system::Config  + pallet_asset_registry::Config{
	/// The overarching event type.
    type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
    type SystemModuleId: Get<ModuleId>;
 //   type AssetId: Parameter + Member + Into<u32> + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;

    type Currency: MultiCurrencyExtended<Self::AccountId, CurrencyId = AssetId, Balance = Balance, Amount = Amount>
            + MultiReservableCurrency<Self::AccountId>;
        

}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		
		// Mint liquidity by adding a liquidity in a pair
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn mint_liquidity(origin, token0: AssetId, amount0: Balance, token1: AssetId, amount1: Balance) -> dispatch::DispatchResult {
            let one: Balance = 1;
            let minimum_liquidity = Balance::from(one);
            let sender = ensure_signed(origin)?;
            ensure!(token0 != token1, Error::<T>::IdenticalIdentifier);
            // Burn assets from user to deposit to reserves
            T::Currency::transfer(token0, &sender,  &Self::account_id(), amount0)?;
            T::Currency::transfer(token1, &sender,  &Self::account_id(), amount1)?;
            let zero_bal: Balance = 0;

            match Pairs::get((token0.clone(), token1.clone())) {
                // create pair if lpt does not exist
                None => {
                    let mut lptoken_amount: Balance = math::sqrt::<T>(amount0 * amount1);
                    lptoken_amount = lptoken_amount.checked_sub(minimum_liquidity).expect("Integer overflow");
                    // Issue LPtoken
                    let lptoken_id: AssetId = <pallet_asset_registry::Pallet<T>>::get_or_create_asset((*b"lptoken").to_vec())?.into();
                   
                    // Deposit assets to the reserve
                    Self::_set_reserves(token0, token1, amount0, amount1, lptoken_id);
                    // Set pairs for swap lookup
                    Self::_set_pair(token0, token1, lptoken_id);
                    Self::_set_rewards(token0, token1, lptoken_id);
                    // Mint LPtoken to the sender
                    T::Currency::deposit(lptoken_id, &sender, lptoken_amount)?;
                    Self::deposit_event(Event::CreatePair(token0, token1, lptoken_id));
                    Ok(())
                },
                // when lpt exists and total supply is bigger than 0
                Some(lpt) if T::Currency::total_issuance(lpt) > Zero::zero() => {
                    let total_supply = T::Currency::total_issuance(lpt);
                    let mut reserves = Self::reserves(lpt);
                    let thousand: Balance = 1000;
                    if token0 > token1 {
                        ensure!(math::absdiff::<T>(reserves.0/reserves.1 * amount0, amount1) < amount0.checked_div(thousand).expect("Divide by zero error"), Error::<T>::K);
                    } else {
                        ensure!(math::absdiff::<T>(reserves.0/reserves.1 * amount1, amount0) < amount0.checked_div(thousand).expect("Divide by zero error"), Error::<T>::K);
                    }
                    let left = amount0.checked_mul(total_supply).expect("Multiplicaiton overflow").checked_div(reserves.0).expect("Divide by zero error");
                    let right = amount1.checked_mul(total_supply).expect("Multiplicaiton overflow").checked_div(reserves.1).expect("Divide by zero error");
                    let lptoken_amount = math::min::<T>(left, right);
                    // Deposit assets to the reserve
                    reserves.0 += amount0;
                    reserves.1 += amount1;
                    Self::_set_reserves(token0, token1, reserves.0, reserves.1, lpt);
                    // Mint LPtoken to the sender
                    T::Currency::deposit(lpt, &sender, lptoken_amount)?;
                    Self::deposit_event(Event::MintedLiquidity(token0, token1, lpt));
                    //Self::_update(&lpt)?;
                    Ok(())
                },
                // <= ?? or just <
                Some(lpt) if T::Currency::total_issuance(lpt) < zero_bal => {
                    Err(Error::<T>::InsufficientLiquidityMinted)?
                },
                Some(_) => Err(Error::<T>::NoneValue)?,
			}
		}
		
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn burn_liquidity(origin, lpt: AssetId, amount: Balance) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut reserves = Self::reserves(lpt);
            let tokens = Self::reward(lpt);
            let total_supply = T::Currency::total_issuance(lpt);

            // Calculate rewards for providing liquidity with pro-rata distribution
            let reward0 = amount.checked_mul(reserves.0).expect("Multiplicaiton overflow").checked_div(total_supply).expect("Divide by zero error");
            let reward1 = amount.checked_mul(reserves.1).expect("Multiplicaiton overflow").checked_div(total_supply).expect("Divide by zero error");

            // Ensure rewards exist
            ensure!(reward0 > Zero::zero() && reward1 > Zero::zero(), Error::<T>::InsufficientLiquidityBurned);

            // Distribute reward to the sender
            T::Currency::withdraw(lpt, &sender, amount)?;
            T::Currency::transfer(tokens.0,  &Self::account_id(), &sender, reward0)?;
            T::Currency::transfer(tokens.1,  &Self::account_id(), &sender, reward1)?;

            // Update reserve when the balance is set
            reserves.0 -= reward0;
            reserves.1 -= reward1;
            Self::_set_reserves(tokens.0, tokens.1, reserves.0, reserves.1, lpt);
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(Event::BurnedLiquidity(lpt, tokens.0, tokens.1));
            // Update price
            //Self::_update(&lpt)?;
            Ok(())
		}
		
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn swap(origin, from: AssetId, amount_in: Balance, to: AssetId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(amount_in > Zero::zero(), Error::<T>::InsufficientAmount);
            // Find pair
            let lpt = Self::pair((from, to));
            ensure!(lpt.is_some(), Error::<T>::InvalidPair);
            let reserves = Self::reserves(lpt.unwrap());
            ensure!(reserves.0 > Zero::zero() && reserves.1 > Zero::zero(), Error::<T>::InsufficientLiquidity);
            let (mut reserve_in, mut reserve_out) = match from > to {
                true => (reserves.1, reserves.0),
                false => (reserves.0, reserves.1)
            };
            // get amount out
            let amount_out = Self::_get_amount_out(amount_in, reserve_in, reserve_out);
            // transfer amount in to system
            T::Currency::transfer(from, &sender,  &Self::account_id(), amount_in)?;
            // transfer swapped amount
            T::Currency::transfer(to,  &Self::account_id(), &sender, amount_out)?;
            // update reserves
            reserve_in += amount_in;
            reserve_out -= amount_out;
            Self::_set_reserves(from, to, reserve_in, reserve_out, lpt.unwrap());
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(Event::Swap(from, amount_in, to, amount_out));
            // Update price
            //Self::_update(&lpt.unwrap())?;
            Ok(())
        }

	}
}

decl_event! {
	pub enum Event 
	{
        /// Pair between two assets is created. \[token0, token1, lptoken]
		CreatePair(AssetId, AssetId, AssetId),
		/// An asset is swapped to another asset. \[token0, amount_in, token1, amount_out]
		Swap(AssetId, Balance, AssetId, Balance),
		/// Liquidity is minted. \[token0, token1, lptoken]
		MintedLiquidity(AssetId, AssetId, AssetId),
		/// Liquidity is burned. \[lptoken, token0, token1]
		BurnedLiquidity(AssetId, AssetId, AssetId),
		/// Sync oracle. \[price0, price1]
        SyncOracle(FixedU128, FixedU128),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
        /// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// Balance should be non-zero
        BalanceZero,
        /// Not the creator of the asset
        NotTheCreator,
        /// Not the approver for the account
        NotApproved,
        /// Created by System
		CreatedBySystem,
		/// No value
		NoneValue,
		/// Insufficient balance
		InSufficientBalance,
		/// Pair already exists
		PairExists,
		/// Lp token id already exists
		LptExists,
		/// Invalid pair
		InvalidPair,
		/// Pair with identical identifiers
		IdenticalIdentifier,
		/// Insufficient liquidity minted
		InsufficientLiquidityMinted,
		/// Insufficient liquidity burned
		InsufficientLiquidityBurned,
		/// Insufficient output amount for swap
		InsufficientOutputAmount,
		/// Insufficient amont for swap
		InsufficientAmount,
		/// Insufficiient liquidity for swap
        InsufficientLiquidity,
        /// The ratio does not match from previous K
        K,

	}
}

decl_storage! {
	trait Store for Module<T: Config> as Assets {
		/// Market storage
		//pub LastBlockTimestamp get(fn last_block_timestamp): T::Moment;
        // Accumulated price data for each pair. key is lptoken identifier
        pub LastAccumulativePrice get(fn last_cumulative_price): map hasher(blake2_128_concat) AssetId => (FixedU128, FixedU128);
        pub Rewards get(fn reward): map hasher(blake2_128_concat) AssetId => (AssetId, AssetId);
        pub Reserves get(fn reserves): map hasher(blake2_128_concat) AssetId => (Balance, Balance);
        pub Pairs get(fn pair): map hasher(blake2_128_concat) (AssetId, AssetId) => Option<AssetId>;
	}
}

// The main implementation block for the module.
impl<T: Config> Module<T> {
    
    pub fn account_id() -> T::AccountId  {
		<T as Config>::SystemModuleId::get().into_account()
	}

	// Market methods
	pub fn _set_reserves(
        token0: AssetId,
        token1: AssetId,
        amount0: Balance,
        amount1: Balance,
        lptoken: AssetId,
    ) {
        match token0 > token1 {
            true => {
                Reserves::insert(lptoken, (amount1, amount0));
            }
            _ => {
                Reserves::insert(lptoken, (amount0, amount1));
            }
        }
    }

    fn _set_pair(token0: AssetId, token1: AssetId, lptoken: AssetId) {
        Pairs::insert((token0, token1), lptoken);
        Pairs::insert((token1, token0), lptoken);
    }
    
	fn _set_rewards(
        token0: AssetId, token1: AssetId, lptoken: AssetId
    ) {
        match token0 > token1 {
            true => {
                Rewards::insert(lptoken, (token1, token0));
            }
            _ => {
                Rewards::insert(lptoken, (token0, token1));
            }
        }
    }

    pub fn to_u256(value: Balance) -> U256 {
        U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(value))
    }

	pub fn _get_amount_out(
        amount_in: Balance,
        reserve_in: Balance,
        reserve_out: Balance,
    ) -> Balance {
        let amount_in_256 = Self::to_u256(amount_in);
        let reserve_in_256 = Self::to_u256(reserve_in);
        let reserve_out_256 = Self::to_u256(reserve_out);
        let amount_in_with_fee = amount_in_256
            .checked_mul(U256::from(997))
            .expect("Multiplication overflow");
        let numerator = amount_in_with_fee
            .checked_mul(reserve_out_256)
            .expect("Multiplication overflow");
        let denominator = reserve_in_256
            .checked_mul(U256::from(1000))
            .expect("Multiplication overflow")
            .checked_add(amount_in_with_fee)
            .expect("Overflow");
        Balance::unique_saturated_from(numerator.checked_div(denominator).expect("divided by zero").as_u128())
    }
	/* 

	// TODO: Reimplement TWAP so that checked calculation does not lose values
	fn _update(pair: &T::AssetId) -> dispatch::DispatchResult {
        let block_timestamp = <timestamp::Module<T>>::get() % T::Moment::from(2u32.pow(32));
        let time_elapsed = block_timestamp - Self::last_block_timestamp();
        let reserves = Self::reserves(pair);
        if time_elapsed > Zero::zero() && reserves.0 != Zero::zero() && reserves.1 != Zero::zero() {
            let reserve0 = FixedU128::saturating_from_integer(reserves.0.saturated_into());
            let reserve1 = FixedU128::saturating_from_integer(reserves.1.saturated_into());
            let price0_cumulative_last = reserve1.checked_div(&reserve0).unwrap()
                * FixedU128::saturating_from_integer(time_elapsed.saturated_into());
            let price1_cumulative_last = reserve0.checked_div(&reserve1).unwrap()
                * FixedU128::saturating_from_integer(time_elapsed.saturated_into());
            <LastAccumulativePrice<T>>::insert(
                &pair,
                (price0_cumulative_last.clone(), price1_cumulative_last.clone()),
            );
            <LastBlockTimestamp<T>>::put(block_timestamp);
            Self::deposit_event(RawEvent::SyncOracle(
                price0_cumulative_last,
                price1_cumulative_last,
            ));
        }
        Ok(())
    }
    */
}


