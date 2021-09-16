use crate::{mock::*, LegalOfficerCase, uuid::UUID};
use frame_support::{assert_ok};
use sp_core::{Encode, Decode};

#[test]
fn it_creates_default_uuid_with_zero_bytes() {
	let uuid: UUID = Default::default();
	for i in 0..32 {
		assert!(uuid.bytes()[i] == 42);
	}
}

#[test]
fn it_encodes_uuid() {
	let uuid: UUID = Default::default();
	let expected_code = [42; 32];
	uuid.using_encoded(|ref slice| {
		assert_eq!(slice, &expected_code);
	});
}

#[test]
fn it_decodes_uuid() {
	let expected_uuid: UUID = UUID::new(vec![41; 32]).ok().unwrap();
	let mut code: &[u8] = &[41; 32];
	assert_eq!(UUID::decode(&mut code).ok(), Some(expected_uuid));
}

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		let loc_id: UUID = Default::default();
		assert_ok!(LogionLoc::create_loc(Origin::signed(1), loc_id));
		assert_eq!(LogionLoc::loc(loc_id), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId> {
			owner: 1
		}));
	});
}
