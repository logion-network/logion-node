use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn asset_creation_works() {
    new_test_ext().execute_with(|| {
        let asset_id = vec![1, 2, 3];
        let asset = pallet_logion_nft::Asset {
            id: asset_id.clone(),
            name: "asset".as_bytes().to_vec()
        };
        assert_ok!(LogionNft::create_asset(Origin::signed(1), asset.clone()));
        assert_eq!(LogionNft::asset_by_id(asset_id), asset);
    });
}
