use crate::{mock::*, LegalOfficerCase, MetadataItem};
use frame_support::{assert_ok, assert_err};
use crate::Error;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		assert_eq!(LogionLoc::loc(0), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId, <Test as crate::Config>::Hash> {
			owner: 1,
			requester: 2,
			metadata: vec![],
			hashes: vec![]
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
fn it_fails_adding_metadata_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_err!(LogionLoc::add_metadata(Origin::signed(2), 0, metadata.clone()), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_adds_hash() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		let hash = BlakeTwo256::hash_of(&"test".as_bytes().to_vec());
		assert_ok!(LogionLoc::add_hash(Origin::signed(1), 0, hash.clone()));
		let loc = LogionLoc::loc(0).unwrap();
		assert_eq!(loc.hashes[0], hash);
	});
}

#[test]
fn it_fails_adding_hash_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), 0, 2));
		let hash = BlakeTwo256::hash_of(&"test".as_bytes().to_vec());
		assert_err!(LogionLoc::add_hash(Origin::signed(2), 0, hash.clone()), Error::<Test>::Unauthorized);
	});
}
