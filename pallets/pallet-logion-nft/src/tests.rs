use crate::mock::*;
use frame_support::{assert_ok, dispatch};
use frame_support::storage::StorageMap;
use pallet_logion_nft::NewAsset;

const ISSUER: <Test as frame_system::Config>::AccountId = 1;
type Hash = <Test as frame_system::Config>::Hash;

#[test]
fn asset_issuance() {
	new_test_ext().execute_with(|| {
		let new_asset = given_new_asset();
		let issuance_result = when_issuing(&new_asset);
		then_success(&issuance_result);
		let asset_hash = existing_asset_hash();
		then_issuer_has_token(&asset_hash);
		then_expected_asset_exists(&asset_hash, &new_asset);
	});
}

fn given_new_asset() -> NewAsset {
	NewAsset {
		name: "asset".as_bytes().to_vec()
	}
}

fn when_issuing(new_asset: &NewAsset) -> dispatch::DispatchResult {
	LogionNft::issue_asset(Origin::signed(ISSUER), new_asset.clone())
}

fn then_success(result: &dispatch::DispatchResult) {
	assert_ok!(result);
}

fn then_issuer_has_token(asset_hash: &Hash) {
	let tokens = LogionNft::token_by_account(ISSUER);
	assert!(tokens.contains(asset_hash));
}

fn then_expected_asset_exists(asset_hash: &Hash, new_asset: &NewAsset) {
	let asset = LogionNft::asset_by_id(asset_hash);
	assert_eq!(asset.name, new_asset.name);
	assert_eq!(asset.issuer, ISSUER);
}

fn existing_asset_hash() -> Hash {
	let tokens = LogionNft::token_by_account(1);
	tokens[0]
}

#[test]
fn token_burning() {
	new_test_ext().execute_with(|| {
		given_existing_asset();
		let asset_hash = existing_asset_hash();
		let burning_result = when_burning_token(&asset_hash);
		then_success(&burning_result);
		then_issuer_has_not_token(&asset_hash);
		then_asset_does_not_exist(&asset_hash);
	});
}

fn given_existing_asset() {
	let new_asset = given_new_asset();
	when_issuing(&new_asset).unwrap();
}

fn when_burning_token(asset_hash: &Hash) -> dispatch::DispatchResult {
	LogionNft::burn_token(Origin::signed(1), asset_hash.clone())
}

fn then_issuer_has_not_token(asset_hash: &Hash) {
	let tokens = LogionNft::token_by_account(ISSUER);
	assert!(!tokens.contains(asset_hash));
}

fn then_asset_does_not_exist(asset_hash: &Hash) {
	assert!(!pallet_logion_nft::AssetById::<Test>::contains_key(&asset_hash));
}