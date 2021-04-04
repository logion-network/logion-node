#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::traits::Vec;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_support::traits::Box;
use sp_runtime::traits::Hash;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

type AssetName = Vec<u8>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Asset<AccountId> {
	name: AssetName,
	issuer: AccountId,
}

type AssetOf<T> = Asset<<T as frame_system::Config>::AccountId>;

decl_storage! {
	trait Store for Module<T: Config> as LogionNft {

		pub AssetById get(fn asset_by_id):
			map hasher(blake2_128_concat) T::Hash => AssetOf<T>;

		pub TokenByAccount get(fn token_by_account):
			map hasher(blake2_128_concat) T::AccountId => Vec<T::Hash>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		Hash = <T as frame_system::Config>::Hash,
	{
		TokenIssued(Hash, AccountId),
		TokenBurned(Hash, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		AssetAlreadyExists,
		NoTokenToBurn,
		NotIssuer,
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct NewAsset {
	name: AssetName,
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn issue_asset(origin, new_asset: NewAsset) -> dispatch::DispatchResult {
			let issuer = ensure_signed(origin)?;

			let asset = AssetOf::<T> {
				name: new_asset.name,
				issuer: issuer.clone(),
			};
			let asset_id = T::Hashing::hash(&asset_data(&asset));
			if AssetById::<T>::contains_key(&asset_id) {
				Err(Error::<T>::AssetAlreadyExists)?
			} else {
				AssetById::<T>::insert::<T::Hash, AssetOf<T>>(asset_id, asset);
				let mut account_assets = TokenByAccount::<T>::get(&issuer).clone();
				account_assets.push(asset_id.clone());
				TokenByAccount::<T>::insert::<T::AccountId, Vec<T::Hash>>(issuer.clone(), account_assets);
				Self::deposit_event(RawEvent::TokenIssued(asset_id, issuer));
				Ok(())
			}
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn burn_token(origin, asset_hash: T::Hash) -> dispatch::DispatchResult {
			let issuer = ensure_signed(origin)?;
			let mut account_tokens = TokenByAccount::<T>::get(&issuer).clone();
			let tokens_in_account = account_tokens.len();
			account_tokens.retain(|hash: &T::Hash| *hash != asset_hash);
			let tokens_left = account_tokens.len();
			if tokens_in_account == tokens_left {
				Err(Error::<T>::NoTokenToBurn)?
			} else {
				let asset = Self::asset_by_id(&asset_hash);
				if asset.issuer != issuer {
					Err(Error::<T>::NotIssuer)?
				} else {
					AssetById::<T>::remove::<T::Hash>(asset_hash.clone());
					TokenByAccount::<T>::insert::<T::AccountId, Vec<T::Hash>>(issuer.clone(), account_tokens);
					Self::deposit_event(RawEvent::TokenBurned(asset_hash, issuer));
					Ok(())
				}
			}
		}
	}
}

pub fn asset_data<AccountId: Encode>(asset: &Asset<AccountId>) -> Box<Vec<u8>> {
	let mut data = Box::new(Vec::new());
	data.extend_from_slice(&asset.name[..]);
	data.extend_from_slice(&asset.issuer.encode());
	return data;
}
