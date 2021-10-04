use crate::{mock::*, LegalOfficerCase, MetadataItem};
use frame_support::{assert_ok, assert_err};
use crate::Error;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;

const LOC_ID: u32 = 0;
const LOC_OWNER: u64 = 1;
const LOC_REQUESTER: u64 = 2;

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId, <Test as crate::Config>::Hash> {
			owner: LOC_OWNER,
			requester: LOC_REQUESTER,
			metadata: vec![],
			hashes: vec![],
			closed: false,
		}));
	});
}

#[test]
fn it_adds_metadata() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_ok!(LogionLoc::add_metadata(Origin::signed(LOC_OWNER), LOC_ID, metadata.clone()));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert_eq!(loc.metadata[0], metadata);
	});
}

#[test]
fn it_fails_adding_metadata_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_err!(LogionLoc::add_metadata(Origin::signed(LOC_REQUESTER), LOC_ID, metadata.clone()), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_adding_metadata_when_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_err!(LogionLoc::add_metadata(Origin::signed(LOC_OWNER), LOC_ID, metadata.clone()), Error::<Test>::CannotMutate);
	});
}

fn create_closed_loc() {
	assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
	assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER), LOC_ID));
}

#[test]
fn it_adds_hash() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		let hash = BlakeTwo256::hash_of(&"test".as_bytes().to_vec());
		assert_ok!(LogionLoc::add_hash(Origin::signed(LOC_OWNER), LOC_ID, hash.clone()));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert_eq!(loc.hashes[0], hash);
	});
}

#[test]
fn it_fails_adding_hash_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		let hash = BlakeTwo256::hash_of(&"test".as_bytes().to_vec());
		assert_err!(LogionLoc::add_hash(Origin::signed(LOC_REQUESTER), LOC_ID, hash.clone()), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_adding_hash_for_when_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		let hash = BlakeTwo256::hash_of(&"test".as_bytes().to_vec());
		assert_err!(LogionLoc::add_hash(Origin::signed(LOC_OWNER), LOC_ID, hash.clone()), Error::<Test>::CannotMutate);
	});
}

#[test]
fn it_closes_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER), LOC_ID));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert!(loc.closed);
	});
}

#[test]
fn it_fails_closing_loc_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER), LOC_ID, LOC_REQUESTER));
		assert_err!(LogionLoc::close(Origin::signed(LOC_REQUESTER), LOC_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_closing_loc_for_already_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		assert_err!(LogionLoc::close(Origin::signed(LOC_OWNER), LOC_ID), Error::<Test>::AlreadyClosed);
	});
}
