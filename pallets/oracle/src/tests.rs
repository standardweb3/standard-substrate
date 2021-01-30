
#![cfg(test)]

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(!<Chainlink>::operator(1));
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(),RawEvent::OperatorRegistered(1));
		assert!(<Chainlink>::operator(1));
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_ok());
		assert!(!<Chainlink>::operator(1));
		assert_eq!(last_event(),RawEvent::OperatorUnregistered(1));
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::unregister_operator(Origin::signed(1)).is_err());
		assert!(!<Chainlink>::operator(1));
	});

}

#[test]
fn initiate_requests() {

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 0, module2::Call::<Test>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 1, module2::Call::<Test>::callback(vec![]).into()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		assert!(<Chainlink>::callback(Origin::signed(3), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert_eq!(last_event(),RawEvent::OperatorRegistered(1));

		let parameters = ("a", "b");
		let data = parameters.encode();
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, data.clone(), 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		assert_eq!(last_event(),RawEvent::OracleRequest(1, vec![], 0, 2, 1, data.clone(), "Chainlink.callback".into(), 2));

		let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
		assert_eq!("a", std::str::from_utf8(&r).unwrap());

		let result = 10;
		assert!(<Chainlink>::callback(Origin::signed(1), 0, result.encode()).is_ok());
		assert_eq!(module2::Result::get(), result);
	});

}

#[test]
pub fn on_finalize() {

	new_test_ext().execute_with(|| {
		assert!(<Chainlink>::register_operator(Origin::signed(1)).is_ok());
		assert!(<Chainlink>::initiate_request(Origin::signed(2), 1, vec![], 1, vec![], 2, module2::Call::<Test>::callback(vec![]).into()).is_ok());
		<Chainlink as OnFinalize<u64>>::on_finalize(20);
		// Request has been killed, too old
		assert!(<Chainlink>::callback(Origin::signed(1), 0, 10.encode()).is_err());
	});

}