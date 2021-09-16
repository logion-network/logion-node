use crate::{mock::*, LegalOfficerCase};
use frame_support::{assert_ok};

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0));
		assert_eq!(LogionLoc::loc(0), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId> {
			owner: 1
		}));
	});
}
