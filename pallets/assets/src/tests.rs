use crate::{Error, mock::*};
use crate::*;
use frame_support::{assert_ok, assert_noop};



#[test]
fn issuing_asset_units_to_issuer_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Assets::balance(1), 0);
	});
}

#[test]
fn querying_total_supply_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::mint(1, 100));
		assert_eq!(Assets::balance(1), 100);
		assert_ok!(Assets::transfer(Origin::signed(1), 2, 50));
		assert_eq!(Assets::balance(1), 50);
		assert_eq!(Assets::balance(2), 50);
		assert_ok!(Assets::transfer(Origin::signed(2), 3, 31));
		assert_eq!(Assets::balance(1), 50);
		assert_eq!(Assets::balance(2), 19);
		assert_eq!(Assets::balance(3), 31);
		// assert_eq!(Assets::total_supply(0), 69);
	});
}

#[test]
fn transferring_amount_above_available_balance_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::mint(1, 100));
		assert_eq!(Assets::balance(1), 100);
		assert_ok!(Assets::transfer(Origin::signed(1), 2, 50));
		assert_eq!(Assets::balance(1), 50);
		assert_eq!(Assets::balance(2), 50);
	});
}

#[test]
fn transferring_amount_more_than_available_balance_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::mint(1, 100));
		assert_eq!(Assets::balance(1), 100);
		assert_ok!(Assets::transfer(Origin::signed(1), 2, 50));
		assert_eq!(Assets::balance(1), 50);
		assert_eq!(Assets::balance(2), 50);
		assert_ok!(Assets::burn(1, 50));
		assert_eq!(Assets::balance(1), 0);
		assert_noop!(Assets::transfer(Origin::signed(1), 1, 50), Error::<Test>::BalanceLow);
	});
}

#[test]
fn transferring_less_than_one_unit_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::mint(1, 100));
		assert_eq!(Assets::balance(1), 100);
		assert_noop!(Assets::transfer(Origin::signed(1), 2, 0), Error::<Test>::AmountZero);
	});
}

#[test]
fn transferring_more_units_than_total_supply_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::mint(1, 100));
		assert_eq!(Assets::balance(1), 100);
		assert_noop!(Assets::transfer(Origin::signed(1), 2, 101), Error::<Test>::BalanceLow);
	});
}

