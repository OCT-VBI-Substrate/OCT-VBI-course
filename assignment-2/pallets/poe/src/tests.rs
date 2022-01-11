use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use crate::Student;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
        let student = Student { id: vec![0; 10], name: vec![1; 10], age: 20 };

		assert_ok!(PoE::create_student(Origin::signed(1), student.id.clone(), student.name.clone(), student.age));

		assert_eq!(PoE::delete_student(Origin::signed(1), student.id, student.name, student.age), Ok(()));

        //assert_eq!(PoE::transfer_student(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let student = Student { id: vec![0; 10], name: vec![1; 10], age: 20 };
		assert_noop!(PoE::delete_student(Origin::signed(1), student.id, student.name, student.age), Error::<Test>::StudentNotExist);

	});
}