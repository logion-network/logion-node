use frame_support::{assert_err, assert_ok};
use frame_support::error::BadOrigin;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;

use logion_shared::LocQuery;

use crate::{File, LegalOfficerCase, LocLink, LocType, MetadataItem, mock::*};
use crate::Error;

const LOC_ID: u32 = 0;
const OTHER_LOC_ID: u32 = 1;

#[test]
fn it_creates_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase::<<Test as frame_system::Config>::AccountId, <Test as crate::Config>::Hash, <Test as crate::Config>::LocId> {
			owner: LOC_OWNER1,
			requester: LOC_REQUESTER,
			metadata: vec![],
			files: vec![],
			closed: false,
			loc_type: LocType::Transaction,
			links: vec![],
			void_info: None,
			replacer_of: None
		}));
	});
}

#[test]
fn it_makes_existing_loc_void() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_ok!(LogionLoc::make_void(Origin::signed(LOC_OWNER1), LOC_ID));

		let void_info = LogionLoc::loc(LOC_ID).unwrap().void_info;
		assert!(void_info.is_some());
		assert!(!void_info.unwrap().replacer.is_some());
	});
}

#[test]
fn it_makes_existing_loc_void_and_replace_it() {
	new_test_ext().execute_with(|| {
		create_closed_loc();

		const REPLACER_LOC_ID: u32 = OTHER_LOC_ID;
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), REPLACER_LOC_ID, LOC_REQUESTER, LocType::Transaction));

		assert_ok!(LogionLoc::make_void_and_replace(Origin::signed(LOC_OWNER1), LOC_ID, REPLACER_LOC_ID));

		let void_info = LogionLoc::loc(LOC_ID).unwrap().void_info;
		assert!(void_info.is_some());
		let replacer: Option<u32> = void_info.unwrap().replacer;
		assert!(replacer.is_some());
		assert_eq!(replacer.unwrap(), REPLACER_LOC_ID);

		let replacer_loc = LogionLoc::loc(REPLACER_LOC_ID).unwrap();
		assert!(replacer_loc.replacer_of.is_some());
		assert_eq!(replacer_loc.replacer_of.unwrap(), LOC_ID)
	});
}

#[test]
fn it_adds_metadata() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let metadata = MetadataItem {
			name: vec![1, 2, 3],
			value: vec![4, 5, 6],
		};
		assert_ok!(LogionLoc::add_metadata(Origin::signed(LOC_OWNER1), LOC_ID, metadata.clone()));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert_eq!(loc.metadata[0], metadata);
	});
}

#[test]
fn it_fails_adding_metadata_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
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
		assert_err!(LogionLoc::add_metadata(Origin::signed(LOC_OWNER1), LOC_ID, metadata.clone()), Error::<Test>::CannotMutate);
	});
}

fn create_closed_loc() {
	assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
	assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER1), LOC_ID));
}

#[test]
fn it_adds_file() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let file = File {
			hash: BlakeTwo256::hash_of(&"test".as_bytes().to_vec()),
			nature: "test-file-nature".as_bytes().to_vec()
		};
		assert_ok!(LogionLoc::add_file(Origin::signed(LOC_OWNER1), LOC_ID, file.clone()));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert_eq!(loc.files[0], file);
	});
}

#[test]
fn it_fails_adding_file_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let file = File {
			hash: BlakeTwo256::hash_of(&"test".as_bytes().to_vec()),
			nature: "test-file-nature".as_bytes().to_vec()
		};
		assert_err!(LogionLoc::add_file(Origin::signed(LOC_REQUESTER), LOC_ID, file.clone()), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_adding_file_when_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		let file = File {
			hash: BlakeTwo256::hash_of(&"test".as_bytes().to_vec()),
			nature: "test-file-nature".as_bytes().to_vec()
		};
		assert_err!(LogionLoc::add_file(Origin::signed(LOC_OWNER1), LOC_ID, file.clone()), Error::<Test>::CannotMutate);
	});
}

#[test]
fn it_adds_link() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), OTHER_LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let link = LocLink {
			id: OTHER_LOC_ID,
			nature: "test-link-nature".as_bytes().to_vec()
		};
		assert_ok!(LogionLoc::add_link(Origin::signed(LOC_OWNER1), LOC_ID, link.clone()));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert_eq!(loc.links[0], link);
	});
}

#[test]
fn it_fails_adding_link_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), OTHER_LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let link = LocLink {
			id: OTHER_LOC_ID,
			nature: "test-link-nature".as_bytes().to_vec()
		};
		assert_err!(LogionLoc::add_link(Origin::signed(LOC_REQUESTER), LOC_ID, link.clone()), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_adding_link_when_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), OTHER_LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let link = LocLink {
			id: OTHER_LOC_ID,
			nature: "test-link-nature".as_bytes().to_vec()
		};
		assert_err!(LogionLoc::add_link(Origin::signed(LOC_OWNER1), LOC_ID, link.clone()), Error::<Test>::CannotMutate);
	});
}

#[test]
fn it_fails_adding_wrong_link() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		let link = LocLink {
			id: OTHER_LOC_ID,
			nature: "test-link-nature".as_bytes().to_vec()
		};
		assert_err!(LogionLoc::add_link(Origin::signed(LOC_OWNER1), LOC_ID, link.clone()), Error::<Test>::LinkedLocNotFound);
	});
}

#[test]
fn it_closes_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER1), LOC_ID));
		let loc = LogionLoc::loc(LOC_ID).unwrap();
		assert!(loc.closed);
	});
}

#[test]
fn it_fails_closing_loc_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_err!(LogionLoc::close(Origin::signed(LOC_REQUESTER), LOC_ID), Error::<Test>::Unauthorized);
	});
}

#[test]
fn it_fails_closing_loc_for_already_closed() {
	new_test_ext().execute_with(|| {
		create_closed_loc();
		assert_err!(LogionLoc::close(Origin::signed(LOC_OWNER1), LOC_ID), Error::<Test>::AlreadyClosed);
	});
}

#[test]
fn it_links_locs_to_account() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Transaction));
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), OTHER_LOC_ID, LOC_REQUESTER, LocType::Identity));
		assert!(LogionLoc::account_locs(LOC_REQUESTER).is_some());
		assert!(LogionLoc::account_locs(LOC_REQUESTER).unwrap().len() == 2);
		assert_eq!(LogionLoc::account_locs(LOC_REQUESTER).unwrap()[0], LOC_ID);
		assert_eq!(LogionLoc::account_locs(LOC_REQUESTER).unwrap()[1], OTHER_LOC_ID);
	});
}

#[test]
fn it_fails_creating_loc_for_unauthorized_caller() {
	new_test_ext().execute_with(|| {
		assert_err!(LogionLoc::create_loc(Origin::signed(LOC_REQUESTER), LOC_ID, LOC_REQUESTER, LocType::Transaction), BadOrigin);
	});
}

#[test]
fn it_detects_existing_identity_loc() {
	new_test_ext().execute_with(|| {
		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER1), LOC_ID, LOC_REQUESTER, LocType::Identity));
		assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER1), LOC_ID));

		assert_ok!(LogionLoc::create_loc(Origin::signed(LOC_OWNER2), OTHER_LOC_ID, LOC_REQUESTER, LocType::Identity));
		assert_ok!(LogionLoc::close(Origin::signed(LOC_OWNER2), OTHER_LOC_ID));

		let legal_officers = Vec::from([LOC_OWNER1, LOC_OWNER2]);
		assert!(LogionLoc::has_closed_identity_locs(&LOC_REQUESTER, &legal_officers));
	});
}
