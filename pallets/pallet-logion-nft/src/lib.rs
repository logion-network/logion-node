#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::traits::Vec;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

type AssetId = Vec<u8>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Asset {
    id: AssetId,
    name: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Config> as LogionNft {

        AssetById get(fn asset_by_id):
            map hasher(blake2_128_concat) AssetId => Asset;

        TokenByAccount get(fn token_by_account):
            map hasher(blake2_128_concat) T::AccountId => Vec<AssetId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        TokenIssued(AssetId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        NoneValue,
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn issue_asset(origin, asset: Asset) -> dispatch::DispatchResult {
            let issuer = ensure_signed(origin)?;

            let asset_id = asset.id.clone();
            if AssetById::contains_key(&asset_id) {
                Err(Error::<T>::StorageOverflow)?
            } else {
                AssetById::insert(&asset_id, asset);
				let mut account_assets = TokenByAccount::<T>::get(&issuer).clone();
				account_assets.push(asset_id.clone());
				TokenByAccount::<T>::insert::<T::AccountId, Vec<AssetId>>(issuer.clone(), account_assets);
                Self::deposit_event(RawEvent::TokenIssued(asset_id, issuer));
                Ok(())
            }
        }
    }
}
