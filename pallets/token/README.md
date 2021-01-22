 # Subswap Module

 An automated market maker module extended from the [asset](../asset/Module.html) module.

 ## Overview

 The Subswap module provides functionality for management and exchange of fungible asset classes
 with a fixed supply, including:

 * Liquidity provider token issuance
 * Compensation for providing liquidity
 * Automated liquidity provisioning
 * Asset exchange

 To use it in your runtime, you need to implement the subswap [`Trait`](./trait.Trait.html).

 The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.

 ### Terminology

 * **Liquidity provider token:** The creation of a new asset by providing liquidity between two fungible assets. Liquidity provider token act as the share of the pool and gets the profit created from exchange fee.
 * **Asset exchange:** The process of an account transferring an asset to exchange with other kind of fungible asset.
 * **Fungible asset:** An asset whose units are interchangeable.
 * **Non-fungible asset:** An asset for which each unit has unique characteristics.

 ### Goals

 The Subswap system in Substrate is designed to make the following possible:

 * Reward liquidity providers with tokens to receive exchanges fees which is proportional to their contribution.
 * Swap assets with automated market price equation(e.g. X*Y=K or curve function from Kyber, dodoex, etc).
 * Issue an fungible asset which can be backed with opening exchange with other assets 

 ## Interface

 ### Dispatchable Functions

 * `issue` - Issues the total supply of a new fungible asset to the account of the caller of the function.
 * `mint` - Mints the asset to the account in the argument with the requested amount from the caller. Caller must be the creator of the asset.
 * `burn` - Burns the asset from the caller by the amount in the argument 
 * `transfer` - Transfers an `amount` of units of fungible asset `id` from the balance of
 the function caller's account (`origin`) to a `target` account.
 * `destroy` - Destroys the entire holding of a fungible asset `id` associated with the account
 that called the function.
 * `mint_liquidity` - Mints liquidity token by adding deposits to a certain pair for exchange. The assets must have different identifier.
 * `burn_liquidity` - Burns liquidity token for a pair and receives each asset in the pair.  
 * `swap` - Swaps from one asset to the another, paying 0.3% fee to the liquidity providers.

 Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.

 ### Public Functions

 * `balance` - Get the balance of the account with the asset id
 * `total_supply` - Get the total supply of an asset.
 * `mint_from_system` - Mint asset from the system to an account, increasing total supply.
 * `burn_from_system` - Burn asset from the system to an account, decreasing total supply.
 * `transfer_from_system - Transfer asset from an account to the system with no change in total supply.
 * `transfer_to_system - Transfer asset from system to the user with no chang in total supply.
 * `issue_from_system` - Issue asset from system 
 * `swap` - Swap one asset to another asset
 
 Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available functions.

 ## Usage

 The following example shows how to use the Subswap module in your runtime by exposing public functions to:

 * Issue and manage a new fungible asset.
 * Query the fungible asset holding balance of an account.
 * Query the total supply of a fungible asset that has been issued.
 * Manage existing asset for other business logic

 ### Prerequisites

 Import the Subswap module and types and derive your runtime's configuration traits from the Assets module trait.

 ### Simple Code Snippet

 ```rust,ignore
 use subswap;
 use pallet_balances as balances;
 use frame_support::{decl_module, dispatch, ensure};
 use frame_system::ensure_signed;

 pub trait Trait: subswap::Trait + balances::Trait {
 
  }

 decl_module! {
 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
 		pub fn trade(origin, token0: T::AssetId, amount0: <T as balances::Trait>::Balance, token1: T::AssetId) -> dispatch::DispatchResult {
 			let sender = ensure_signed(origin).map_err(|e| e.as_str())?;

             let amount_out = subswap::Module<T>::swap(&token0, &amount0, &token1); 
 			
 			Self::deposit_event(RawEvent::Trade(token0, amount0, token1, amount_out));
 			Ok(())
 		}
 	}
 }
 ```

 ## Assumptions

 Below are assumptions that must be held when using this module.  If any of
 them are violated, the behavior of this module is undefined.

 * The total count of assets should be less than
   `Trait::AssetId::max_value()`.

 ## Related Modules

 * [`System`](../frame_system/index.html)
 * [`Support`](../frame_support/index.html)