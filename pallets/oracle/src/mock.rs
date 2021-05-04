
#![cfg(test)]

use crate::*;
use sp_core::H256;
use frame_support::{impl_outer_origin, impl_outer_event, parameter_types, weights::Weight};
use crate::sp_api_hidden_includes_decl_storage::hidden_include::StorageValue;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill, ModuleId
};
use pallet_chainlink::CallbackWithParameter;
use frame_system as system;
use pallet_balances as balances;

impl_outer_origin! {
	pub enum Origin for Test {}
}

pub mod oracle {
	// Re-export needed for `impl_outer_event!`.
	pub use super::super::*;
}

impl_outer_event! {
	pub enum TestEvent for Test {
		system<T>,
		pallet_balances<T>,
		pallet_chainlink<T>,
	}
}

pub(crate) type Balance = u128;

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Header = Header;
	type Event = ();
	type Lookup = IdentityLookup<Self::AccountId>;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
	pub const AssetModuleId: ModuleId = ModuleId(*b"stnd/ast");
	pub const ValidityPeriod: u64 = 10;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type Event = ();
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl pallet_standard_token::Config for Test {
	type WeightInfo = ();
	type ModuleId = AssetModuleId;
	type Event = ();
	type AssetId = u64;
}

impl pallet_chainlink::Config for Test {
	type Event = ();
	type Currency = pallet_balances::Module<Test>;
	type Callback = module2::Call<Test>;
	type ValidityPeriod = ValidityPeriod;
}

impl Trait for Test {
	type Event = ();
}

impl module2::Trait for Test {
}

//TODO: make mockup for chainlink integration. 
//BLOCKER: chainlink overrides existing events for runtime modules. 
pub mod module2 {
	use super::*;

	pub trait Trait: frame_system::Trait {}

	frame_support::decl_module! {
		pub struct Module<T: Trait> for enum Call
			where origin: <T as frame_system::Config>::Origin
		{
			#[weight = 0]
			pub fn callback(_origin, result: Vec<u8>) -> frame_support::dispatch::DispatchResult {
				let r : u128 = u128::decode(&mut &result[..]).map_err(|err| err.what())?;
				<Result>::put(r);
				Ok(())
			}
		}
	}

	frame_support::decl_storage! {
		trait Store for Module<T: Trait> as TestStorage {
			pub Result: u128;
		}
	}

	impl <T: Trait> CallbackWithParameter for Call<T> {
		fn with_result(&self, result: Vec<u8>) -> Option<Self> {
			match *self {
				Call::callback(_) => Some(Call::callback(result)),
				_ => None
			}
		}
	}

}

pub type System  = system::Module<Test>;
type Chainlink = pallet_chainlink::Module<Test>;
pub type Balances = balances::Module<Test>;
pub type Token = pallet_standard_token::Module<Test>;
pub type Market = Module<Test>;


pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test>{
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![(0, 100000), (1, 100000), (2, 100000)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn last_event() -> RawEvent<u128, u64> {
	System::events().into_iter().map(|r| r.event)
		.filter_map(|e| {
			if let TestEvent::chainlink(inner) = e { Some(inner) } else { None }
		})
		.last()
		.unwrap()
}
