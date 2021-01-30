
#![cfg(test)]

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
	
	#[test]
	fn issuing_asset_units_to_issuer_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
		});
	}
	
	#[test]
	fn querying_total_supply_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_ok!(Token::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Token::balance(0, 1), 50);
			assert_eq!(Token::balance(0, 2), 50);
			assert_ok!(Token::transfer(Origin::signed(2), 0, 3, 31));
			assert_eq!(Token::balance(0, 1), 50);
			assert_eq!(Token::balance(0, 2), 19);
			assert_eq!(Token::balance(0, 3), 31);
			assert_ok!(Token::destroy(Origin::signed(3), 0));
			assert_eq!(Token::total_supply(0), 69);
		});
	}
	
	#[test]
	fn transferring_amount_above_available_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_ok!(Token::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Token::balance(0, 1), 50);
			assert_eq!(Token::balance(0, 2), 50);
		});
	}
	
	#[test]
	fn transferring_amount_more_than_available_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_ok!(Token::transfer(Origin::signed(1), 0, 2, 50));
			assert_eq!(Token::balance(0, 1), 50);
			assert_eq!(Token::balance(0, 2), 50);
			assert_ok!(Token::destroy(Origin::signed(1), 0));
			assert_eq!(Token::balance(0, 1), 0);
			assert_noop!(Token::transfer(Origin::signed(1), 0, 1, 50), Error::<Test>::BalanceLow);
		});
	}
	
	#[test]
	fn transferring_less_than_one_unit_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_noop!(Token::transfer(Origin::signed(1), 0, 2, 0), Error::<Test>::AmountZero);
		});
	}
	
	#[test]
	fn transferring_more_units_than_total_supply_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_noop!(Token::transfer(Origin::signed(1), 0, 2, 101), Error::<Test>::BalanceLow);
		});
	}
	
	#[test]
	fn destroying_asset_balance_with_positive_balance_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 1), 100);
			assert_ok!(Token::destroy(Origin::signed(1), 0));
		});
	}
	
	#[test]
	fn destroying_asset_balance_with_zero_balance_should_not_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Token::issue(Origin::signed(1), 100));
			assert_eq!(Token::balance(0, 2), 0);
			assert_noop!(Token::destroy(Origin::signed(2), 0), Error::<Test>::BalanceZero);
		});
	}
	
