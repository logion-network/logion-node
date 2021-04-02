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
		let asset_id = <Test as frame_system::Config>::Hashing::hash(&asset_data);
		assert_ok!(LogionNft::issue_asset(Origin::signed(1), asset.clone()));
		assert_eq!(LogionNft::asset_by_id(&asset_id), asset);
		assert!(LogionNft::token_by_account(1).contains(&asset_id));
	});
}
