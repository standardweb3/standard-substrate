use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[cfg(test)]
mod tests {
	use super::*;

	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use sp_core::H256;
	use sp_runtime::{Perbill, traits::{BlakeTwo256, IdentityLookup}, testing::Header};

	impl_outer_origin! {
		pub enum Origin for Test where system = frame_system {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}
	impl frame_system::Trait for Test {
		type BaseCallFilter = ();
		type Origin = Origin;
		type Index = u64;
		type Call = ();
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type MaximumExtrinsicWeight = MaximumBlockWeight;
		type AvailableBlockRatio = AvailableBlockRatio;
		type MaximumBlockLength = MaximumBlockLength;
		type Version = ();
		type PalletInfo = ();
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
	}
	impl Trait for Test {
		type Event = ();
		type Balance = u128;
		type AssetId = u64;
		type ModuleId = AssetModuleId;
	}
	impl From<u128> for Balance {
		fn from(value: u128) -> Balance {
			return Self::from(value);
		}
	}
	type Assets = Module<Test>;

	fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn issuing_asset_units_to_issuer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
		});
	}

	#[test]
	fn querying_total_supply_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
			assert_ok!(Assets::transfer(Origin::signed(2), 0, 3, 31));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 19);
			assert_eq!(Assets::balance(0, 3), 31);
			assert_ok!(Assets::destroy(Origin::signed(3), 0));
			assert_eq!(Assets::total_supply(0), 69);
		});
	}

	#[test]
	fn transferring_amount_above_available_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
		});
	}

	#[test]
	fn transferring_amount_more_than_available_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Assets::balance(0, 1), 50);
			assert_eq!(Assets::balance(0, 2), 50);
			assert_ok!(Assets::destroy(Origin::signed(1), 0));
			assert_eq!(Assets::balance(0, 1), 0);
			assert_noop!(Assets::transfer(Origin::signed(1), 0, 1, 50), Error::<Test>::BalanceLow);
		});
	}

	#[test]
	fn transferring_less_than_one_unit_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_noop!(Assets::transfer(Origin::signed(1), 0, 2, 0), Error::<Test>::AmountZero);
		});
	}

	#[test]
	fn transferring_more_units_than_total_supply_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_noop!(Assets::transfer(Origin::signed(1), 0, 2, 101), Error::<Test>::BalanceLow);
		});
	}

	#[test]
	fn destroying_asset_balance_with_positive_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 1), 100);
			assert_ok!(Assets::destroy(Origin::signed(1), 0));
		});
	}

	#[test]
	fn destroying_asset_balance_with_zero_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::issue(Origin::signed(1), 100));
			assert_eq!(Assets::balance(0, 2), 0);
			assert_noop!(Assets::destroy(Origin::signed(2), 0), Error::<Test>::BalanceZero);
		});
	}
}