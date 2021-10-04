#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::codec::{Decode, Encode};
use frame_support::traits::Vec;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct MetadataItem {
	name: Vec<u8>,
	value: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct LegalOfficerCase<AccountId, Hash> {
	owner: AccountId,
	requester: AccountId,
	metadata: Vec<MetadataItem>,
	hashes: Vec<Hash>,
	closed: bool,
}

pub type LegalOfficerCaseOf<T> = LegalOfficerCase<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash>;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use codec::HasCompact;
	use super::*;
	pub use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// LOC identifier
		type LocId: Member + Parameter + Default + Copy + HasCompact;

		/// Type for hashes stored in LOCs
		type Hash: Member + Parameter + Default + Copy;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// All LOCs indexed by ID.
	#[pallet::storage]
	#[pallet::getter(fn loc)]
	pub type LocMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, LegalOfficerCaseOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::LocId = "LocId")]
	pub enum Event<T: Config> {
		/// Issued upon LOC creation. [locId]
		LocCreated(T::LocId),
		/// Issued when LOC is closed. [locId]
		LocClosed(T::LocId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The LOC ID has already been used.
		AlreadyExists,
		/// Target LOC does not exist
		NotFound,
		/// Unauthorized LOC operation
		Unauthorized,
		/// Occurs when trying to mutate a closed LOC
		CannotMutate,
		/// Occurs when trying to close an already closed LOC
		AlreadyClosed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Creates a new LOC
		#[pallet::weight(T::WeightInfo::create_loc())]
		pub fn create_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			requester: T::AccountId
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let loc = LegalOfficerCaseOf::<T> {
					owner: who.clone(),
					requester: requester.clone(),
					metadata: Vec::new(),
					hashes: Vec::new(),
					closed: false,
				};
				<LocMap<T>>::insert(loc_id, loc);

				Self::deposit_event(Event::LocCreated(loc_id));
				Ok(().into())
			}
		}

		/// Add LOC metadata
		#[pallet::weight(T::WeightInfo::add_metadata())]
		pub fn add_metadata(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			item: MetadataItem
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			if ! <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::NotFound)?
			} else {
				let loc = <LocMap<T>>::get(&loc_id).unwrap();
				if loc.owner != who {
					Err(Error::<T>::Unauthorized)?
				} else if loc.closed {
					Err(Error::<T>::CannotMutate)?
				} else {
					<LocMap<T>>::mutate(loc_id, |loc| {
						let mutable_loc = loc.as_mut().unwrap();
						mutable_loc.metadata.push(item);
					});
					Ok(().into())
				}
			}
		}

		/// Add hash to LOC
		#[pallet::weight(T::WeightInfo::add_hash())]
		pub fn add_hash(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			hash: <T as pallet::Config>::Hash
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			if ! <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::NotFound)?
			} else {
				let loc = <LocMap<T>>::get(&loc_id).unwrap();
				if loc.owner != who {
					Err(Error::<T>::Unauthorized)?
				} else if loc.closed {
					Err(Error::<T>::CannotMutate)?
				} else {
					<LocMap<T>>::mutate(loc_id, |loc| {
						let mutable_loc = loc.as_mut().unwrap();
						mutable_loc.hashes.push(hash);
					});
					Ok(().into())
				}
			}
		}

		/// Close LOC.
		#[pallet::weight(T::WeightInfo::close())]
		pub fn close(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			if ! <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::NotFound)?
			} else {
				let loc = <LocMap<T>>::get(&loc_id).unwrap();
				if loc.owner != who {
					Err(Error::<T>::Unauthorized)?
				} else if loc.closed {
					Err(Error::<T>::AlreadyClosed)?
				} else {
					<LocMap<T>>::mutate(loc_id, |loc| {
						let mutable_loc = loc.as_mut().unwrap();
						mutable_loc.closed = true;
					});

					Self::deposit_event(Event::LocClosed(loc_id));
					Ok(().into())
				}
			}
		}
	}
}
