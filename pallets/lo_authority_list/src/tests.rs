use crate::mock::*;
use frame_support::{assert_err, assert_ok, error::BadOrigin, traits::EnsureOrigin};

const LEGAL_OFFICER_ID: u64 = 1;
const ANOTHER_ID: u64 = 2;

#[test]
fn it_adds_lo() {
	new_test_ext().execute_with(|| {
		assert_ok!(LoAuthorityList::add_legal_officer(Origin::signed(MANAGER), LEGAL_OFFICER_ID));
		assert!(LoAuthorityList::legal_officer_set(LEGAL_OFFICER_ID).is_some());
	});
}

#[test]
fn it_removes_lo() {
	new_test_ext().execute_with(|| {
		assert_ok!(LoAuthorityList::add_legal_officer(Origin::signed(MANAGER), LEGAL_OFFICER_ID));
		assert_ok!(LoAuthorityList::remove_legal_officer(Origin::signed(MANAGER), LEGAL_OFFICER_ID));
		assert!(LoAuthorityList::legal_officer_set(LEGAL_OFFICER_ID).is_none());
	});
}

#[test]
fn it_fails_adding_if_not_manager() {
	new_test_ext().execute_with(|| {
		assert_err!(LoAuthorityList::add_legal_officer(Origin::signed(0), LEGAL_OFFICER_ID), BadOrigin);
	});
}

#[test]
fn it_fails_removing_if_not_manager() {
	new_test_ext().execute_with(|| {
		assert_err!(LoAuthorityList::remove_legal_officer(Origin::signed(0), LEGAL_OFFICER_ID), BadOrigin);
	});
}

#[test]
fn it_ensures_origin_ok_as_expected() {
	new_test_ext().execute_with(|| {
		assert_ok!(LoAuthorityList::add_legal_officer(Origin::signed(MANAGER), LEGAL_OFFICER_ID));
		assert_ok!(LoAuthorityList::try_origin(Origin::signed(LEGAL_OFFICER_ID)));
	});
}

#[test]
fn it_ensures_origin_err_as_expected() {
	new_test_ext().execute_with(|| {
		assert_ok!(LoAuthorityList::add_legal_officer(Origin::signed(MANAGER), LEGAL_OFFICER_ID));
		let result = LoAuthorityList::try_origin(Origin::signed(ANOTHER_ID));
		assert!(result.err().is_some());
	});
}
