use crate::{mock::*, LegalOfficerCase, MetadataItem};
use frame_support::{assert_ok, assert_err};
use crate::Error;

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		assert_eq!(LogionLoc::loc(0), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId> {
			owner: 1,
			requester: 2,
			metadata: vec![]
		}));
	});
}

#[test]
fn it_adds_metadata() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_ok!(LogionLoc::add_metadata(Origin::signed(1), 0, metadata.clone()));
		let loc = LogionLoc::loc(0).unwrap();
		assert_eq!(loc.metadata[0], metadata);
	});
}

#[test]
fn it_fails_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_err!(LogionLoc::add_metadata(Origin::signed(2), 0, metadata.clone()), Error::<Test>::Unauthorized);
	});
}
