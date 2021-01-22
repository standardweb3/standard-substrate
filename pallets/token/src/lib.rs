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
//! 		pub fn trade(origin, token0: <T as Trait>::AssetId, amount0: BalanceOf<T>, token1: <T as Trait>::AssetId) -> dispatch::DispatchResult {
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

use frame_support::{Parameter, decl_module, decl_event, decl_storage, decl_error, ensure, dispatch,
    traits::{Currency, ReservableCurrency, WithdrawReasons, ExistenceRequirement, Get},
};
use sp_runtime::traits::{AtLeast32BitUnsigned, Member, Zero, StaticLookup};
use frame_system::{ensure_signed};
use sp_runtime::traits::One;
use codec::{Encode, Decode, HasCompact};
use sp_runtime::{traits::AccountIdConversion, ModuleId};
pub mod weights;
pub use weights::WeightInfo;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// The balance type of this module.
pub type BalanceOf<T> =
	<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// The module configuration trait.
pub trait Trait: frame_system::Trait{
    /// The Module account for burning assets
    type ModuleId: Get<ModuleId>;

    /// The currency trait.
    type Currency: ReservableCurrency<Self::AccountId>;

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The arithmetic type of asset identifier.
    type AssetId: Member + Parameter + Default + Copy + HasCompact + AtLeast32BitUnsigned;

    type WeightInfo: WeightInfo;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		const ModuleId: ModuleId = T::ModuleId::get();

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
		#[weight = T::WeightInfo::issue()]
		fn issue(origin, #[compact] total: BalanceOf<T>) {
			let origin = ensure_signed(origin)?;
			let id = Self::next_asset_id();
			<NextAssetId<T>>::mutate(|id| {
                *id += One::one();
            });

			<Balances<T>>::insert((id, &origin), total);
			<TotalSupply<T>>::insert(id, total);
			<Creator<T>>::insert(id, &origin);

			Self::deposit_event(RawEvent::Issued(id, origin, total));
        }
        
        /// Destroy any assets of `id` owned by `origin`.
		///
		/// # <weight>
		/// - `O(1)`
		/// - 1 storage mutation (codec `O(1)`).
		/// - 1 storage deletion (codec `O(1)`).
		/// - 1 event.
		/// # </weight>
		#[weight = T::WeightInfo::destroy()]
		fn destroy(origin, #[compact] id: <T as Trait>::AssetId) {
			let origin = ensure_signed(origin)?;
			let balance = <Balances<T>>::take((id, &origin));
			ensure!(!balance.is_zero(), Error::<T>::BalanceZero);

			<TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
			Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
		}

		/// Mint any assets of `id` owned by `origin`.
        ///
        /// # <weight>
        /// - `O(1)`
        /// - 1 storage mutation (codec `O(1)`).
        /// - 1 storage deletion (codec `O(1)`).
        /// - 1 event.
        /// # </weight>
        #[weight = T::WeightInfo::mint()]
        fn mint(origin,
             #[compact] id: <T as Trait>::AssetId,
            target: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: BalanceOf<T>
        ){
            let origin = ensure_signed(origin)?;
            let target = T::Lookup::lookup(target)?;
            let creator = <Creator<T>>::get(id);
            ensure!(origin == creator, Error::<T>::NotTheCreator);
            ensure!(!amount.is_zero(), Error::<T>::AmountZero);

            Self::deposit_event(RawEvent::Minted(id, target.clone(), amount));
            <Balances<T>>::mutate((id, target), |balance| *balance += amount);
        }


        /// Burn any assets of `id` owned by `origin`.
        ///
        /// # <weight>
        /// - `O(1)`
        /// - 1 storage mutation (codec `O(1)`).
        /// - 1 storage deletion (codec `O(1)`).
        /// - 1 event.
        /// # </weight>
        #[weight = T::WeightInfo::burn()]
        fn burn(origin,
            #[compact] id: <T as Trait>::AssetId,
           target: <T::Lookup as StaticLookup>::Source,
           #[compact] amount: BalanceOf<T>
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
		#[weight = T::WeightInfo::transfer()]
		fn transfer(origin,
			#[compact] id: <T as Trait>::AssetId,
			target: <T::Lookup as StaticLookup>::Source,
			#[compact] amount: BalanceOf<T>
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
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		Balance = BalanceOf<T>,
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
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
        /// Transfer amount should be non-zero but amount is zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// Sender has zero balance
        BalanceZero,
        /// Sender is not the creator of the asset
        NotTheCreator,
        /// TODO: add allowance for token
        /// Sender is not the approver for the account
        NotApproved,
        /// Created by System
		CreatedBySystem,
		/// Null value in the registry
		NoneValue,
		/// Sender has Insufficient balance
		InSufficientBalance,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Token {
		/// The number of units of assets held by any given account.
		Balances: map hasher(blake2_128_concat) (<T as Trait>::AssetId, T::AccountId) => BalanceOf<T>;
		/// The next asset identifier up for grabs.
		pub NextAssetId get(fn next_asset_id): <T as Trait>::AssetId;
		/// The total unit supply of an asset.
		TotalSupply: map hasher(twox_64_concat) <T as Trait>::AssetId => BalanceOf<T>;
        /// Creator of the asset
        Creator: map hasher(blake2_128_concat) <T as Trait>::AssetId => T::AccountId;
    }
    add_extra_genesis {
		config(preregistered):
			Vec<BalanceOf<T>>;
		build(|config: &GenesisConfig<T>| {
			for &ref balance in &config.preregistered {
                let module_account = <Module<T>>::account_id();
				let _ = <Module<T>>::issue(
					T::Origin::from(Some(module_account.clone()).into()),
					balance.clone()
				);
			}
		});
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
	pub fn balance(id: <T as Trait>::AssetId, who: <T as frame_system::Trait>::AccountId) -> BalanceOf<T> {
		<Balances<T>>::get((id, who))
    }
    
 

	/// Get the total supply of an asset `id`.
	pub fn total_supply(id: <T as Trait>::AssetId) -> BalanceOf<T> {
		<TotalSupply<T>>::get(id)
	}

	pub fn mint_from_system(
        id: &<T as Trait>::AssetId,
        target: &<T as frame_system::Trait>::AccountId,
        amount: &BalanceOf<T>,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::Minted(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            T::Currency::deposit_creating(target, *amount);
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
            <TotalSupply<T>>::mutate(*id, |supply| *supply += *amount);
        }
        Ok(())
    }

    pub fn burn_from_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &BalanceOf<T>,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::Burned(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let reason = WithdrawReasons::all();
            T::Currency::withdraw(target, *amount, reason, ExistenceRequirement::AllowDeath)?;
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance -= *amount);
            <TotalSupply<T>>::mutate(*id, |supply| *supply -= *amount);
        }
        Ok(())
    }

    pub fn transfer_from_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &BalanceOf<T>,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::TransferredFromSystem(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let module_account = Self::account_id();
            T::Currency::transfer(&module_account, target, *amount, ExistenceRequirement::AllowDeath)?;
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance += *amount);
        }
        Ok(())
    }

    pub fn transfer_to_system(
        id: &<T as Trait>::AssetId,
        target: &T::AccountId,
        amount: &BalanceOf<T>,
    ) -> dispatch::DispatchResult {
        ensure!(!amount.is_zero(), Error::<T>::AmountZero);
        Self::deposit_event(RawEvent::TransferredToSystem(*id, target.clone(), *amount));
        if *id == Zero::zero() {
            let module_account = Self::account_id();
            T::Currency::transfer(target, &module_account, *amount, ExistenceRequirement::AllowDeath)?;
        } else {
            <Balances<T>>::mutate((*id, target.clone()), |balance| *balance -= *amount);
        }
        Ok(())
    }
}