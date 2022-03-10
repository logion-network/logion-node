use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;

#[test]
fn it_requests_call_if_not_legal_officer() {
	new_test_ext().execute_with(|| {
		let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
		assert_ok!(Vault::request_call(Origin::signed(USER_ID), vec![LEGAL_OFFICER1, LEGAL_OFFICER2], call_hash.to_fixed_bytes(), 10000));
	});
}

#[test]
fn it_fails_requesting_call_if_legal_officer() {
	new_test_ext().execute_with(|| {
		let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
		assert_err!(Vault::request_call(Origin::signed(LEGAL_OFFICER1), vec![LEGAL_OFFICER1, LEGAL_OFFICER2], call_hash.to_fixed_bytes(), 10000), Error::<Test>::WrongInitiator);
	});
}

#[test]
fn it_fails_requesting_call_if_not_two_legal_officers() {
	new_test_ext().execute_with(|| {
		let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
		assert_err!(Vault::request_call(Origin::signed(USER_ID), vec![LEGAL_OFFICER1], call_hash.to_fixed_bytes(), 10000), Error::<Test>::InvalidSignatories);
	});
}

#[test]
fn it_fails_requesting_call_if_not_all_legal_officers() {
	new_test_ext().execute_with(|| {
		let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
		assert_err!(Vault::request_call(Origin::signed(USER_ID), vec![LEGAL_OFFICER1, ANOTHER_USER_ID], call_hash.to_fixed_bytes(), 10000), Error::<Test>::InvalidSignatories);
	});
}

#[test]
fn it_approves_call_if_two_other_signatories() {
	new_test_ext().execute_with(|| {
		let call = "call-bytes".as_bytes().to_vec();
		let timepoint = Default::default();
		assert_ok!(Vault::approve_call(Origin::signed(LEGAL_OFFICER1), vec![USER_ID, LEGAL_OFFICER2], call, timepoint, 10000));
	});
}
