// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure};
use frame_system::{ensure_signed, ensure_root };
use sp_runtime::{ DispatchResult, DispatchError};
use sp_std::prelude::*;

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
		pub fn register_operator(origin, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			//ensure!(!<Operators<T>>::get(&who), Error::<T>::OperatorAlreadyRegistered);

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
			let results = match Self::asset_price(_id) {
				Some(mut x) => {
				  x.push(_price);
				  x
				},
				_ => {
				  vec!{_price}
				}
			  };
			Prices::insert(_id, results);
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
        
        pub Prices get(fn asset_price): map hasher(blake2_128_concat) AssetId =>  Option<Vec<Balance>>;

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
			Some(mut reports) => {
				// get median value
				reports.sort();
				let mid = reports.len() / 2;
				let median = reports[mid];
				return Ok(median)
			},
			None => {
				return Err(DispatchError::from(crate::Error::<T>::PriceDoesNotExist).into());
			}
		}
	}
}
