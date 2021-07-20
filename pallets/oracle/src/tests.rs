#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_oracle_provider_works() {
	new_test_ext().execute_with(|| {
		// Adding operator requires root.
		assert_noop!(
			Oracle::register_operator(Origin::signed(11), 1, 1u64),
			BadOrigin
		);
		assert_ok!(Oracle::register_operator(Origin::root(), 1, 2));
	})
}

#[test]
fn oracle_report_works() {
	new_test_ext().execute_with(|| {
		let provider = 1u64;
		// Adding operator requires root.
		assert_noop!(
			Oracle::register_operator(Origin::signed(11), 1, provider),
			BadOrigin
		);
		assert_ok!(Oracle::register_operator(Origin::root(), 1, provider));

		assert_ok!(Oracle::report(Origin::signed(provider.into()), 1, 1, 2));

		assert_eq!(Oracle::asset_price(1), Some(vec! {0,2,0,0,0}));
	})
}

#[test]
fn oracle_slash_works() {
	new_test_ext().execute_with(|| {
		let provider_1 = 1u64;
		let provider_2 = 2u64;
		let provider_3 = 3u64;
		let provider_4 = 4u64;
		let provider_5 = 5u64;
		let slasher = 6u64;
		// Adding operator requires root.
		assert_noop!(
			Oracle::register_operator(Origin::signed(11), 1, provider_1),
			BadOrigin
		);
		// setup batch of oracle providers
		assert_ok!(Oracle::register_operator(Origin::root(), 0, provider_1));
		assert_ok!(Oracle::register_operator(Origin::root(), 1, provider_2));
		assert_ok!(Oracle::register_operator(Origin::root(), 2, provider_3));
		assert_ok!(Oracle::register_operator(Origin::root(), 3, provider_4));
		assert_ok!(Oracle::register_operator(Origin::root(), 4, provider_5));

		// setup batch of oracle values [1,2,1,2,1]
		assert_ok!(Oracle::report(Origin::signed(provider_1.into()), 0, 1, 1));
		assert_ok!(Oracle::report(Origin::signed(provider_2.into()), 1, 1, 2));
		assert_ok!(Oracle::report(Origin::signed(provider_3.into()), 2, 1, 1));
		assert_ok!(Oracle::report(Origin::signed(provider_4.into()), 3, 1, 2));
		assert_ok!(Oracle::report(Origin::signed(provider_5.into()), 4, 1, 1));
		assert_eq!(Oracle::asset_price(1), Some(vec! {1,2,1,2,1}));

		// and one of providers submit an manipulated value which goes out of acceptable error range
		assert_ok!(Oracle::report(Origin::signed(provider_1.into()), 0, 1, 4));
		assert_eq!(Oracle::asset_price(1), Some(vec! {4,2,1,2,1}));
		// detect outlier and slash the provider
		let dd = Oracle::asset_price(1).unwrap();
		assert_eq!(Oracle::test(dd), vec![1,1,2,2,4]);
		assert_ok!(Oracle::slash(Origin::signed(slasher), 0, 1));
		// slot for oracle submission is now empty
		assert_eq!(Oracle::provider_at(0), 0);
	})
}

#[test]
fn oracle_excludes_zeros_and_return_median() {
	new_test_ext().execute_with(|| {
		let provider_1 = 1u64;
		let provider_2 = 2u64;
		let provider_3 = 3u64;
		let provider_4 = 4u64;
		let provider_5 = 5u64;
		let slasher = 6u64;
		// Adding operator requires root.
		assert_noop!(
			Oracle::register_operator(Origin::signed(11), 1, provider_1),
			BadOrigin
		);
		// setup batch of oracle providers
		assert_ok!(Oracle::register_operator(Origin::root(), 0, provider_1));
		assert_ok!(Oracle::register_operator(Origin::root(), 1, provider_2));
		assert_ok!(Oracle::register_operator(Origin::root(), 2, provider_3));
		assert_ok!(Oracle::register_operator(Origin::root(), 3, provider_4));
		assert_ok!(Oracle::register_operator(Origin::root(), 4, provider_5));

		// setup batch of oracle values [0,0,1,2,3]
		assert_ok!(Oracle::report(Origin::signed(provider_1.into()), 0, 1, 0));
		assert_ok!(Oracle::report(Origin::signed(provider_2.into()), 1, 1, 0));
		assert_ok!(Oracle::report(Origin::signed(provider_3.into()), 2, 1, 1));
		assert_ok!(Oracle::report(Origin::signed(provider_4.into()), 3, 1, 2));
		assert_ok!(Oracle::report(Origin::signed(provider_5.into()), 4, 1, 3));
		assert_eq!(Oracle::asset_price(1), Some(vec! {0,0,1,2,3}));

		// and the median should be 2
		assert_eq!(Oracle::get_median(Oracle::asset_price(1).unwrap()), 2);

	})
}
