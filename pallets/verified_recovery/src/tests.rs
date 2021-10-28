use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

#[test]
fn it_creates_recovery_config_if_both_closed() {
	new_test_ext().execute_with(|| {
		assert_ok!(VerifiedRecovery::create_recovery(Origin::signed(USER_ID), vec![LEGAL_OFFICER_CLOSED_ID1, LEGAL_OFFICER_CLOSED_ID2]));
	});
}

#[test]
fn it_fails_creating_recovery_config_if_both_open_or_pending() {
	new_test_ext().execute_with(|| {
		assert_err!(VerifiedRecovery::create_recovery(Origin::signed(USER_ID), vec![LEGAL_OFFICER_PENDING_OR_OPEN_ID1, LEGAL_OFFICER_PENDING_OR_OPEN_ID2]), Error::<Test>::MissingIdentityLoc);
	});
}

#[test]
fn it_fails_creating_recovery_config_if_first_open_or_pending() {
	new_test_ext().execute_with(|| {
		assert_err!(VerifiedRecovery::create_recovery(Origin::signed(USER_ID), vec![LEGAL_OFFICER_PENDING_OR_OPEN_ID1, LEGAL_OFFICER_CLOSED_ID2]), Error::<Test>::MissingIdentityLoc);
	});
}

#[test]
fn it_fails_creating_recovery_config_if_second_open_or_pending() {
	new_test_ext().execute_with(|| {
		assert_err!(VerifiedRecovery::create_recovery(Origin::signed(USER_ID), vec![LEGAL_OFFICER_CLOSED_ID1, LEGAL_OFFICER_PENDING_OR_OPEN_ID2]), Error::<Test>::MissingIdentityLoc);
	});
}
