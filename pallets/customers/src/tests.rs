use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(CustomersModule::create_customer(RuntimeOrigin::signed(1), 0));
		// Read pallet storage and assert an expected result.
		// assert_eq!(CustomersModule::something(), Some(42));
		// Assert that the correct event was deposited
		// System::assert_last_event(Event::CustomerCreated { customer_id: 0, who: 1 });
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(
		// 	CustomersModule::cause_error(RuntimeOrigin::signed(1)),
		// 	Error::<Test>::NoneValue
		// );
	});
}
