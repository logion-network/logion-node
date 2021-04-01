#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_support::codec::{Encode, Decode};
use frame_support::traits::Vec;
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {

    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Asset {
    id: Vec<u8>,
    name: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Config> as LogionNft {

        AssetById get(fn asset_by_id):
            map hasher(blake2_128_concat) Vec<u8> => Asset;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        SomethingStored(Vec<u8>, AccountId),
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
        pub fn create_asset(origin, asset: Asset) -> dispatch::DispatchResult {
            let issuer = ensure_signed(origin)?;

            let asset_id = asset.id.clone();
            if AssetById::contains_key(&asset_id) {
                Err(Error::<T>::StorageOverflow)?
            } else {
                AssetById::insert(&asset_id, asset);
                Self::deposit_event(RawEvent::SomethingStored(asset_id, issuer));
                Ok(())
            }
        }
    }
}
