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
//! 		pub fn trade(origin, token0: <T as Trait>::AssetId, amount0: <T as balances::Trait>::Balance, token1: <T as Trait>::AssetId) -> dispatch::DispatchResult {
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

use frame_support::{Parameter, decl_module, decl_event, decl_storage, decl_error, ensure, dispatch};
use sp_runtime::traits::{AtLeast32Bit, Zero, StaticLookup};
use frame_system::ensure_signed;
use sp_runtime::traits::One;
use pallet_balancess;
use sp_core::U256;
use pallet_timestamp as timestamp;
use sp_runtime::{FixedU128, FixedPointNumber, SaturatedConversion, traits::{UniqueSaturatedInto, UniqueSaturatedFrom}};
use sp_runtime::traits::{CheckedMul, CheckedAdd, CheckedDiv, CheckedSub};
use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
mod math;

/// The module configuration trait.
pub trait Trait: frame_system::Trait + pallet_balancess::Trait + timestamp::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The units in which we record pallet_balancess.
///	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	/// The arithmetic type of asset identifier.
	type AssetId: Parameter + AtLeast32Bit + Default + Copy;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		/// Issue a new class of fungible assets. There are, and will only ever be, `total`
		/// such assets and they'll all belong to the `origin` initially. It will have an
		/// identifier `AssetId` instance: this will be specified in the `Issued` event.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 2 storage writes (condec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn issue(origin, #[compact] total: T::Balance) {
			let origin = ensure_signed(origin)?;
			// save 0 for native currency
			let mut id = Self::next_asset_id();
			if id == Zero::zero() {
				id += One::one();
			}
			<NextAssetId<T>>::mutate(|id| {
                if *id == Zero::zero() {
                    *id += One::one();
                }
                *id += One::one();
            });

			<pallet_balancess<T>>::insert((id, &origin), total);
			<TotalSupply<T>>::insert(id, total);
			<Creator<T>>::insert(id, &origin);

			Self::deposit_event(RawEvent::Issued(id, origin, total));
		}

		/// Mint any assets of `id` owned by `origin`.
        ///
        /// # <weight>
        /// - `O(1)`
        /// - 1 storage mutation (codec `O(1)`).
        /// - 1 storage deletion (codec `O(1)`).
        /// - 1 event.
        /// # </weight>
        #[weight = 0]
        fn mint(origin,
             #[compact] id: <T as Trait>::AssetId,
            target: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: <T as pallet_balancess::Trait>::Balance
        ){
            let origin = ensure_signed(origin)?;
            let target = T::Lookup::lookup(target)?;
            let creator = <Creator<T>>::get(id);
            ensure!(origin == creator, Error::<T>::NotTheCreator);
            ensure!(!amount.is_zero(), Error::<T>::AmountZero);

            Self::deposit_event(RawEvent::Minted(id, target.clone(), amount));
            <pallet_balancess<T>>::mutate((id, target), |balance| *balance += amount);
        }


        /// Burn any assets of `id` owned by `origin`.
        ///
        /// # <weight>
        /// - `O(1)`
        /// - 1 storage mutation (codec `O(1)`).
        /// - 1 storage deletion (codec `O(1)`).
        /// - 1 event.
        /// # </weight>
        #[weight = 0]
        fn burn(origin,
            #[compact] id: <T as Trait>::AssetId,
           target: <T::Lookup as StaticLookup>::Source,
           #[compact] amount: <T as balances::Trait>::Balance
       ){
           let origin = ensure_signed(origin)?;
           let origin_account = (id, origin.clone());
           let origin_balance = <Balances<T>>::get(&origin_account);
           ensure!(!amount.is_zero(), Error::<T>::AmountZero);
           ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

           Self::deposit_event(RawEvent::Burned(id, origin, amount));
           <Balances<T>>::insert(origin_account, origin_balance - amount);
       }

		/// Move some assets from one holder to another.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 static lookup
		/// - 2 storage mutations (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn transfer(origin,
			#[compact] id: <T as Trait>::AssetId,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: T::Balance
		) {
			let origin = ensure_signed(origin)?;
			let origin_account = (id, origin.clone());
			let origin_balance = <Balances<T>>::get(&origin_account);
			let target = T::Lookup::lookup(target)?;
			ensure!(!amount.is_zero(), Error::<T>::AmountZero);
			ensure!(origin_balance >= amount, Error::<T>::BalanceLow);

			Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
			<Balances<T>>::insert(origin_account, origin_balance - amount);
			<Balances<T>>::mutate((id, target), |balance| *balance += amount);
		}

		/// Destroy any assets of `id` owned by `origin`.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 1 storage deletion (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = 0]
		fn destroy(origin, #[compact] id: <T as Trait>::AssetId) {
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, &origin));
			ensure!(!balance.is_zero(), Error::<T>::BalanceZero);

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
		}



		// Market Module functions
		// TODO: Separate this functions as separate module and share same primitives
		
		
		// Mint liquidity by adding a liquidity in a pair
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn mint_liquidity(origin, token0: <T as Trait>::AssetId, amount0: <T as pallet_balances::Trait>::Balance, token1: <T as Trait>::AssetId, amount1: <T as pallet_balances::Trait>::Balance) -> dispatch::DispatchResult {
            let minimum_liquidity = <T as pallet_balances::Trait>::Balance::from(1);
            let sender = ensure_signed(origin)?;
            ensure!(token0 != token1, Error::<T>::IdenticalIdentifier);
            // Burn assets from user to deposit to reserves
            Module::<T>::transfer_to_system(&token0, &sender, &amount0)?;
            Module::<T>::transfer_to_system(&token1, &sender, &amount1)?;
            match Pairs::<T>::get((token0.clone(), token1.clone())) {
                // create pair if lpt does not exist
                None => {
                    let mut lptoken_amount: <T as pallet_balances::Trait>::Balance = math::sqrt::<T>(amount0 * amount1);
                    lptoken_amount = lptoken_amount.checked_sub(&minimum_liquidity).expect("Integer overflow");
                    // Issue LPtoken
                    Module::<T>::issue_from_system(Zero::zero())?;
                    let mut lptoken_id: <T as Trait>::AssetId = NextAssetId::<T>::get();
                    lptoken_id -= One::one();
                    // Deposit assets to the reserve
                    Self::_set_reserves(&token0, &token1, &amount0, &amount1, &lptoken_id);
                    // Set pairs for swap lookup
                    Self::_set_pair(&token0, &token1, &lptoken_id);
                    Self::_set_rewards(&token0, &token1, &lptoken_id);
                    // Mint LPtoken to the sender
                    Module::<T>::mint_from_system(&lptoken_id, &sender, &lptoken_amount)?;
                    Self::deposit_event(RawEvent::CreatePair(token0, token1, lptoken_id));
                    Ok(())
                },
                // when lpt exists and total supply is superset of 0
                Some(lpt) if Module::<T>::total_supply(lpt) > Zero::zero() => {
                    let total_supply = Module::<T>::total_supply(lpt);
                    let mut reserves = Self::reserves(lpt);
                    let left = amount0.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.0).expect("Divide by zero error");
                    let right = amount1.checked_mul(&total_supply).expect("Multiplicaiton overflow").checked_div(&reserves.1).expect("Divide by zero error");
                    let lptoken_amount = math::min::<T>(left, right);
                    // Deposit assets to the reserve
                    reserves.0 += amount0;
                    reserves.1 += amount1;
                    Self::_set_reserves(&token0, &token1, &reserves.0, &reserves.1, &lpt);
                    // Mint LPtoken to the sender
                    Module::<T>::mint_from_system(&lpt, &sender, &lptoken_amount)?;
                    Self::deposit_event(RawEvent::MintedLiquidity(token0, token1, lpt));
                    //Self::_update(&lpt)?;
                    Ok(())
                },
                Some(lpt) if Module::<T>::total_supply(lpt) < <T as pallet_balances::Trait>::Balance::from(0) => {
                    Err(Error::<T>::InsufficientLiquidityMinted)?
                },
                Some(_) => Err(Error::<T>::NoneValue)?,
			}
		}
		
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn burn_liquidity(origin, lpt: <T as Trait>::AssetId, amount: <T as pallet_balances::Trait>::Balance) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            let mut reserves = Self::reserves(lpt);
            let tokens = Self::reward(lpt);
            let total_supply = Module::<T>::total_supply(lpt);

            // Calculate rewards for providing liquidity with pro-rata distribution
            let reward0 = amount.checked_mul(&reserves.0).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");
            let reward1 = amount.checked_mul(&reserves.1).expect("Multiplicaiton overflow").checked_div(&total_supply).expect("Divide by zero error");

            // Ensure rewards exist
            ensure!(reward0 > Zero::zero() && reward1 > Zero::zero(), Error::<T>::InsufficientLiquidityBurned);

            // Distribute reward to the sender
            Module::<T>::burn_from_system(&lpt, &sender, &amount)?;
            Module::<T>::transfer_from_system(&tokens.0, &sender, &reward0)?;
            Module::<T>::transfer_from_system(&tokens.1, &sender, &reward1)?;

            // Update reserve when the balance is set
            reserves.0 -= reward0;
            reserves.1 -= reward1;
            Self::_set_reserves(&tokens.0, &tokens.1, &reserves.0, &reserves.1, &lpt);
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(RawEvent::BurnedLiquidity(lpt, tokens.0, tokens.1));
            // Update price
            //Self::_update(&lpt)?;
            Ok(())
		}
		
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn swap(origin, from: <T as Trait>::AssetId, amount_in: <T as pallet_balances::Trait>::Balance, to: <T as Trait>::AssetId) -> dispatch::DispatchResult {
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
            let amount_out = Self::_get_amount_out(&amount_in, &reserve_in, &reserve_out);
            // transfer amount in to system
            Module::<T>::transfer_to_system(&from, &sender, &amount_in)?;
            // transfer swapped amount
            Module::<T>::transfer_from_system(&to, &sender, &amount_out)?;
            // update reserves
            reserve_in += amount_in;
            reserve_out -= amount_out;
            Self::_set_reserves(&from, &to, &reserve_in, &reserve_out, &lpt.unwrap());
            // Deposit event that the liquidity is burned successfully
            Self::deposit_event(RawEvent::Swap(from, amount_in, to, amount_out));
            // Update price
            //Self::_update(&lpt.unwrap())?;
            Ok(())
        }

	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as pallet_balances::Trait>::Balance,
		<T as Trait>::AssetId,
	{
        /// Some assets were issued. \[asset_id, owner, total_supply\]
        Issued(AssetId, AccountId, Balance),
        /// Some assets were issued by the system(e.g. lpt, pool tokens) \[asset_id, total_supply]
        IssuedBySystem(AssetId, Balance),
        /// Some assets were transferred. \[asset_id, from, to, amount\]
        Transferred(AssetId, AccountId, AccountId, Balance),
        TransferredFromSystem(AssetId, AccountId, Balance),
        TransferredToSystem(AssetId, AccountId, Balance),
        /// Some assets were minted. \[asset_id, owner, balance]
        Minted(AssetId, AccountId, Balance),
        /// Some assets were burned. \[asset_id, owner, balance]
        Burned(AssetId, AccountId, Balance),
        /// Some assets were destroyed. \[asset_id, owner, balance\]
		Destroyed(AssetId, AccountId, Balance),
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
	pub enum Error for Module<T: Trait> {
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
        K,

	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Assets {
		/// The number of units of assets held by any given account.
		Balances: map hasher(blake2_128_concat) (<T as Trait>::AssetId, T::AccountId) => T::Balance;
		/// The next asset identifier up for grabs.
		pub NextAssetId get(fn next_asset_id): <T as Trait>::AssetId;
		/// The total unit supply of an asset.
		///
		/// TWOX-NOTE: `AssetId` is trusted, so this is safe.
		TotalSupply: map hasher(twox_64_concat) <T as Trait>::AssetId => T::Balance;
		Creator: map hasher(blake2_128_concat) <T as Trait>::AssetId => T::AccountId;

		/// Market storage
		/// TODO: decouple this with separate module with defi primitive
		pub LastBlockTimestamp get(fn last_block_timestamp): T::Moment;
        // Accumulated price data for each pair. key is lptoken identifier
        pub LastAccumulativePrice get(fn last_cumulative_price): map hasher(blake2_128_concat) <T as Trait>::AssetId => (FixedU128, FixedU128);
        pub Rewards get(fn reward): map hasher(blake2_128_concat) <T as Trait>::AssetId => (<T as Trait>::AssetId, <T as Trait>::AssetId);
        pub Reserves get(fn reserves): map hasher(blake2_128_concat) <T as Trait>::AssetId => (<T as pallet_balances::Trait>::Balance, <T as pallet_balances::Trait>::Balance);
        pub Pairs get(fn pair): map hasher(blake2_128_concat) (<T as Trait>::AssetId, <T as Trait>::AssetId) => Option<<T as Trait>::AssetId>;
	}
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
	// Module account id
	pub fn account_id() -> <T as frame_system::Trait>::AccountId {
		T::ModuleId::get().into_account()
	}
	// Public immutables

	/// Get the asset `id` balance of `who`.
	pub fn balance(id: <T as Trait>::AssetId, who: <T as frame_system::Trait>::AccountId) -> <T as pallet_balances::Trait>::Balance {
		<Balances<T>>::get((id, who))
    }
    
 

	/// Get the total supply of an asset `id`.
	pub fn total_supply(id: <T as Trait>::AssetId) -> T::Balance {
		<TotalSupply<T>>::get(id)
	}

	pub fn mint_from_system(
        id: &<T as Trait>::AssetId,
        target: &<T as frame_system::Trait>::AccountId,
        amount: &<T as pallet_balances::Trait>::Balance,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::Minted(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let new_free = pallet_balances::Module::<T>::free_balance(target) + *amount;
            pallet_balances::Module::<T>::mutate_account(target, |account| {
                account.free = new_free;

                account.free
            });
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
            <TotalSupply<T>>::mutate(*id, |supply| *supply += *amount);
        }
        Ok(())
    }

    pub fn burn_from_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &T::Balance,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::Burned(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let new_free = pallet_balances::Module::<T>::free_balance(target) - *amount;
            let _free = pallet_balances::Module::<T>::mutate_account(target, |account| {
                account.free = new_free;

                account.free
            });
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance -= *amount);
            <TotalSupply<T>>::mutate(*id, |supply| *supply -= *amount);
        }
        Ok(())
    }

    pub fn transfer_from_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &T::Balance,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::TransferredFromSystem(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let new_free = pallet_balances::Module::<T>::free_balance(target) + *amount;
            let _free = pallet_balances::Module::<T>::mutate_account(target, |account| {
                account.free = new_free;

                account.free
            });
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
        }
        Ok(())
    }

    pub fn transfer_to_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &T::Balance,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::TransferredToSystem(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let new_free = pallet_balances::Module::<T>::free_balance(target) - *amount;
            let _free = pallet_balances::Module::<T>::mutate_account(target, |account| {
                account.free = new_free;

                account.free
            });
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance -= *amount);
        }
        Ok(())
    }

    pub fn issue_from_system(total: T::Balance) -> dispatch::DispatchResult {
        let id = Self::next_asset_id();
        <NextAssetId<T>>::mutate(|id| {
            if *id == Zero::zero() {
                *id += One::one();
            }
            *id += One::one();
        });
        <TotalSupply<T>>::insert(id, total);

        Self::deposit_event(RawEvent::IssuedBySystem(id, total));
        Ok(())
	}
	


	// Market methods
	// TODO: separate these functions into a new module and share primitives with this
	fn _set_reserves(
        token0: &<T as Trait>::AssetId,
        token1: &<T as Trait>::AssetId,
        amount0: &<T as pallet_balances::Trait>::Balance,
        amount1: &<T as pallet_balances::Trait>::Balance,
        lptoken: &<T as Trait>::AssetId,
    ) {
        match *token0 > *token1 {
            true => {
                <Reserves<T>>::insert(*lptoken, (*amount1, *amount0));
            }
            _ => {
                <Reserves<T>>::insert(*lptoken, (*amount0, *amount1));
            }
        }
    }

    fn _set_pair(token0: &<T as Trait>::AssetId, token1: &<T as Trait>::AssetId, lptoken: &<T as Trait>::AssetId) {
        <Pairs<T>>::insert((*token0, *token1), *lptoken);
        <Pairs<T>>::insert((*token1, *token0), *lptoken);
    }
    
	fn _set_rewards(
        token0: &<T as Trait>::AssetId, token1: &<T as Trait>::AssetId, lptoken: &<T as Trait>::AssetId
    ) {
        match *token0 > *token1 {
            true => {
                <Rewards<T>>::insert(*lptoken, (*token1, *token0));
            }
            _ => {
                <Rewards<T>>::insert(*lptoken, (*token0, *token1));
            }
        }
    }

    pub fn to_u256(value: &<T as pallet_balances::Trait>::Balance) -> U256 {
        U256::from(UniqueSaturatedInto::<u128>::unique_saturated_into(*value))
    }

	pub fn _get_amount_out(
        amount_in: &<T as pallet_balances::Trait>::Balance,
        reserve_in: &<T as pallet_balances::Trait>::Balance,
        reserve_out: &<T as pallet_balances::Trait>::Balance,
    ) -> <T as pallet_balances::Trait>::Balance {
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
        <T as pallet_balances::Trait>::Balance::unique_saturated_from(numerator.checked_div(denominator).expect("divided by zero").as_u128())
    }
	

	// TODO: Reimplement TWAP so that checked calculation does not lose values
	fn _update(pair: &<T as Trait>::AssetId) -> dispatch::DispatchResult {
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
}



