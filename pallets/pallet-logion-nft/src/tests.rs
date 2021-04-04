use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::traits::Hash;

#[test]
fn asset_creation_works() {
	new_test_ext().execute_with(|| {
		let asset = pallet_logion_nft::Asset {
			name: "asset".as_bytes().to_vec()
		};
		let asset_data = pallet_logion_nft::asset_data(&asset);
		let asset_hash = <Test as frame_system::Config>::Hashing::hash(&asset_data);
		assert_ok!(LogionNft::issue_asset(Origin::signed(1), asset.clone()));
		assert_eq!(LogionNft::asset_by_id(&asset_hash), asset);
		assert!(LogionNft::token_by_account(1).contains(&asset_hash));
	});
}

#[test]
fn token_burning_works() {
	new_test_ext().execute_with(|| {
		let asset_hash = _given_asset_issued();
		assert_ok!(LogionNft::burn_token(Origin::signed(1), asset_hash.clone()));
		assert!(!LogionNft::token_by_account(1).contains(&asset_hash));
	});
}

fn _given_asset_issued() -> <Test as frame_system::Config>::Hash {
	let asset = pallet_logion_nft::Asset {
		name: "asset".as_bytes().to_vec()
	};
	let asset_data = pallet_logion_nft::asset_data(&asset);
	let asset_hash = <Test as frame_system::Config>::Hashing::hash(&asset_data);
	LogionNft::issue_asset(Origin::signed(1), asset.clone()).unwrap();
	asset_hash
}
