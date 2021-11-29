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

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum LocType {
	Transaction,
	Identity
}

impl Default for LocType {
	fn default() -> LocType {
		return LocType::Transaction;
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct MetadataItem {
	name: Vec<u8>,
	value: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct LocLink<LocId> {
	id: LocId,
	nature: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct File<Hash> {
	hash: Hash,
	nature: Vec<u8>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum LocVoidInfo<LocId> {
	V1 {
		reason: Vec<u8>,
		replacer: Option<LocId>,
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct LegalOfficerCase<AccountId, Hash, LocId> {
	owner: AccountId,
	requester: AccountId,
	metadata: Vec<MetadataItem>,
	files: Vec<File<Hash>>,
	closed: bool,
	loc_type: LocType,
	links: Vec<LocLink<LocId>>,
	void_info: Option<LocVoidInfo<LocId>>,
	replacer_of: Option<LocId>
}

pub type LegalOfficerCaseOf<T> = LegalOfficerCase<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use codec::HasCompact;
	use logion_shared::LocQuery;
	use super::*;
	pub use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// LOC identifier
		type LocId: Member + Parameter + Default + Copy + HasCompact;

		/// Type for hashes stored in LOCs
		type Hash: Member + Parameter + Default + Copy;

		/// The origin (must be signed) which can create a LOC.
		type CreateOrigin: EnsureOrigin<Self::Origin>;

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

	/// Requested LOCs by account ID.
	#[pallet::storage]
	#[pallet::getter(fn account_locs)]
	pub type AccountLocsMap<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, Vec<<T as Config>::LocId>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::LocId = "LocId")]
	pub enum Event<T: Config> {
		/// Issued upon LOC creation. [locId]
		LocCreated(T::LocId),
		/// Issued when LOC is closed. [locId]
		LocClosed(T::LocId),
		/// Issued when LOC is made void. [locId]
		LocVoid(T::LocId),
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
		/// Occurs when trying to link to a non-existent LOC
		LinkedLocNotFound,
		/// Occurs when trying to replace void LOC with a non-existent LOC
		ReplacerLocNotFound,
		/// Occurs when trying to void a LOC already void
		AlreadyVoid,
		/// Occurs when trying to void a LOC by replacing it with an already void LOC
		ReplacerLocAlreadyVoid,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[derive(Encode, Decode, Eq, PartialEq)]
	pub enum StorageVersion {
		V1,
		V2MakeLocVoid,
	}

	// /// Storage version
	// #[pallet::storage]
	// #[pallet::getter(fn pallet_storage_version)]
	// pub type PalletStorageVersion = StorageVersion::V1;

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Creates a new LOC
		#[pallet::weight(T::WeightInfo::create_loc())]
		pub fn create_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			requester: T::AccountId,
			loc_type: LocType,
		) -> DispatchResultWithPostInfo {
			T::CreateOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let loc = LegalOfficerCaseOf::<T> {
					owner: who.clone(),
					requester: requester.clone(),
					metadata: Vec::new(),
					files: Vec::new(),
					closed: false,
					loc_type: loc_type.clone(),
					links: Vec::new(),
					void_info: None,
					replacer_of: None
				};
				<LocMap<T>>::insert(loc_id, loc);

				if <AccountLocsMap<T>>::contains_key(requester.clone()) {
					<AccountLocsMap<T>>::mutate(requester.clone(), |accounts| {
						let list = accounts.as_mut().unwrap();
						list.push(loc_id.clone());
					});
				} else {
					<AccountLocsMap<T>>::insert(requester.clone(), Vec::from([loc_id.clone()]));
				}

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

		/// Add file to LOC
		#[pallet::weight(T::WeightInfo::add_file())]
		pub fn add_file(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			file: File<<T as pallet::Config>::Hash>
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
						mutable_loc.files.push(file);
					});
					Ok(().into())
				}
			}
		}

		/// Add a link to LOC
		#[pallet::weight(T::WeightInfo::add_link())]
		pub fn add_link(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			link: LocLink<T::LocId>
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
				} else if ! <LocMap<T>>::contains_key(&link.id) {
					Err(Error::<T>::LinkedLocNotFound)?
				} else {
					<LocMap<T>>::mutate(loc_id, |loc| {
						let mutable_loc = loc.as_mut().unwrap();
						mutable_loc.links.push(link);
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

		/// Make a LOC void.
		#[pallet::weight(T::WeightInfo::make_void())]
		pub fn make_void(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			reason: Vec<u8>,
			replacer_loc_id: Option<T::LocId>
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			if replacer_loc_id.is_some() {
				let replacer = replacer_loc_id.unwrap();
				if !<LocMap<T>>::contains_key(&replacer) {
					Err(Error::<T>::ReplacerLocNotFound)?
				} else {
					let replacer_loc = <LocMap<T>>::get(&replacer).unwrap();
					if replacer_loc.void_info.is_some() {
						Err(Error::<T>::ReplacerLocAlreadyVoid)?
					}
				}
			}

			if !<LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::NotFound)?
			} else {
				let loc = <LocMap<T>>::get(&loc_id).unwrap();
				if loc.owner != who {
					Err(Error::<T>::Unauthorized)?
				}
				if loc.void_info.is_some() {
					Err(Error::<T>::AlreadyVoid)?
				}
			}

			let loc_void_info = LocVoidInfo::V1 {
				reason,
				replacer:replacer_loc_id
			};
			<LocMap<T>>::mutate(loc_id, |loc| {
				let mutable_loc = loc.as_mut().unwrap();
				mutable_loc.void_info = Some(loc_void_info);
			});
			if replacer_loc_id.is_some() {
				<LocMap<T>>::mutate(replacer_loc_id.unwrap(), |replacer_loc| {
					let mutable_replacer_loc = replacer_loc.as_mut().unwrap();
					mutable_replacer_loc.replacer_of = Some(loc_id);
				});
			}
			Self::deposit_event(Event::LocVoid(loc_id));
			Ok(().into())
		}

	}

	impl<T: Config> LocQuery<<T as frame_system::Config>::AccountId> for Pallet<T> {
		fn has_closed_identity_locs(
			account: &<T as frame_system::Config>::AccountId,
			legal_officers: &Vec<<T as frame_system::Config>::AccountId>
		) -> bool {
			Self::has_closed_identity_loc(account, &legal_officers[0]) && Self::has_closed_identity_loc(account, &legal_officers[1])
		}
	}

	impl<T: Config> Pallet<T> {

		fn has_closed_identity_loc(
			account: &<T as frame_system::Config>::AccountId,
			legal_officer: &<T as frame_system::Config>::AccountId
		) -> bool {
			let value = <AccountLocsMap<T>>::get(account);
			match value {
				Some(loc_ids) => {
					return loc_ids.iter().map(|id| <LocMap<T>>::get(id))
						.filter(|option| option.is_some())
						.map(|some| some.unwrap())
						.find(|loc| loc.owner == *legal_officer && loc.loc_type == LocType::Identity && loc.closed)
						.is_some();
				}
				None => false
			}
		}
	}

	pub mod migration {
		use super::*;

		pub mod v1 {
			use frame_support::codec::{Decode, Encode};
			use frame_support::traits::Vec;

			use crate::{File, LocLink, LocType, MetadataItem, pallet};

			#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
			pub struct LegalOfficerCaseV1<AccountId, Hash, LocId> {
				pub owner: AccountId,
				pub requester: AccountId,
				pub metadata: Vec<MetadataItem>,
				pub files: Vec<File<Hash>>,
				pub closed: bool,
				pub loc_type: LocType,
				pub links: Vec<LocLink<LocId>>
			}

			pub type LegalOfficerCaseOfV1<T> = LegalOfficerCaseV1<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

		}

		pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight {
			use crate::migration::v1::LegalOfficerCaseOfV1;
			debug::RuntimeLogger::init();
			debug::info!("Starting migration...");
			<LocMap<T>>::translate::<LegalOfficerCaseOfV1<T>, _>(
				|_loc_id: T::LocId, loc: LegalOfficerCaseOfV1<T>|{
					debug::info!("Migrating LOC");
					let new_loc = LegalOfficerCaseOf::<T> {
						owner: loc.owner.clone(),
						requester: loc.requester.clone(),
						metadata: loc.metadata.clone(),
						files: loc.files.clone(),
						closed: loc.closed.clone(),
						loc_type: loc.loc_type.clone(),
						links: loc.links.clone(),
						void_info: None,
						replacer_of: None
					};
					Some(new_loc)
				}
			);
			debug::info!("Migration ended.");
			let count = <LocMap<T>>::iter().count();
			T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
		}
	}
}
