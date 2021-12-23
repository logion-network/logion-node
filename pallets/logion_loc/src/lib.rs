#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod migration;

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
pub struct LocVoidInfo<LocId> {
	replacer: Option<LocId>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum Requester<AccountId, LocId> {
	None,
	Account(AccountId),
	Loc(LocId)
}

pub type RequesterOf<T> = Requester<<T as frame_system::Config>::AccountId, <T as Config>::LocId>;

impl<AccountId, LocId> Default for Requester<AccountId, LocId> {

	fn default() -> Requester<AccountId, LocId> {
		Requester::None
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct LegalOfficerCase<AccountId, Hash, LocId> {
	owner: AccountId,
	requester: Requester<AccountId, LocId>,
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

	/// Requested LOCs by logion Identity LOC.
	#[pallet::storage]
	#[pallet::getter(fn identity_loc_locs)]
	pub type IdentityLocLocsMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, Vec<<T as Config>::LocId>>;

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
		/// Occurs when trying to void a LOC by replacing it with a LOC already replacing another LOC
		ReplacerLocAlreadyReplacing,
		/// Occurs when trying to mutate a void LOC
		CannotMutateVoid,
		/// Unexpected requester given LOC type
		UnexpectedRequester,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[derive(Encode, Decode, Eq, PartialEq, Debug)]
	pub enum StorageVersion {
		V1,
		V2MakeLocVoid,
		V3RequesterEnum,
	}

	impl Default for StorageVersion {
		fn default() -> StorageVersion {
			return StorageVersion::V3RequesterEnum;
		}
	}

	/// Storage version
	#[pallet::storage]
	#[pallet::getter(fn pallet_storage_version)]
	pub type PalletStorageVersion<T> = StorageValue<_, StorageVersion, ValueQuery>;

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Creates a new Polkadot Identity LOC i.e. a LOC linking a real identity to an AccountId.
		#[pallet::weight(T::WeightInfo::create_polkadot_identity_loc())]
		pub fn create_polkadot_identity_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			requester_account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::CreateOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let requester = RequesterOf::<T>::Account(requester_account_id.clone());
				let loc = Self::build_open_loc(&who, &requester, LocType::Identity);
	
				<LocMap<T>>::insert(loc_id, loc);
				Self::link_with_account(&requester_account_id, &loc_id);

				Self::deposit_event(Event::LocCreated(loc_id));
				Ok(().into())
			}
		}

		/// Creates a new logion Identity LOC i.e. a LOC describing a real identity not yet linked to an AccountId
		#[pallet::weight(T::WeightInfo::create_logion_identity_loc())]
		pub fn create_logion_identity_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
		) -> DispatchResultWithPostInfo {
			T::CreateOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let requester = RequesterOf::<T>::None;
				let loc = Self::build_open_loc(&who, &requester, LocType::Identity);
				<LocMap<T>>::insert(loc_id, loc);

				Self::deposit_event(Event::LocCreated(loc_id));
				Ok(().into())
			}
		}

		/// Creates a new Polkadot Transaction LOC i.e. a LOC requested with an AccountId
		#[pallet::weight(T::WeightInfo::create_polkadot_transaction_loc())]
		pub fn create_polkadot_transaction_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			requester_account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::CreateOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let requester = RequesterOf::<T>::Account(requester_account_id.clone());
				let loc = Self::build_open_loc(&who, &requester, LocType::Transaction);

				<LocMap<T>>::insert(loc_id, loc);
				Self::link_with_account(&requester_account_id, &loc_id);

				Self::deposit_event(Event::LocCreated(loc_id));
				Ok(().into())
			}
		}

		/// Creates a new logion Transaction LOC i.e. a LOC requested with a logion Identity LOC
		#[pallet::weight(T::WeightInfo::create_logion_transaction_loc())]
		pub fn create_logion_transaction_loc(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			requester_loc_id: T::LocId,
		) -> DispatchResultWithPostInfo {
			T::CreateOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			if <LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				let requester_loc = <LocMap<T>>::get(&requester_loc_id);
				match requester_loc {
					None => Err(Error::<T>::UnexpectedRequester)?,
					Some(loc) =>
						if loc.loc_type != LocType::Identity
							|| match loc.requester { RequesterOf::<T>::None => false, _ => true }
							|| !loc.closed
							|| loc.void_info.is_some() {
							Err(Error::<T>::UnexpectedRequester)?
						} else {
							let requester = RequesterOf::<T>::Loc(requester_loc_id.clone());
							let new_loc = Self::build_open_loc(&who, &requester, LocType::Transaction);
							<LocMap<T>>::insert(loc_id, new_loc);
							Self::link_with_identity_loc(&requester_loc_id, &loc_id);
						},
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
				} else if loc.void_info.is_some() {
					Err(Error::<T>::CannotMutateVoid)?
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
				} else if loc.void_info.is_some() {
					Err(Error::<T>::CannotMutateVoid)?
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
				} else if loc.void_info.is_some() {
					Err(Error::<T>::CannotMutateVoid)?
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
				} else if loc.void_info.is_some() {
					Err(Error::<T>::CannotMutateVoid)?
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
		) -> DispatchResultWithPostInfo {
			Self::do_make_void(origin, loc_id, None)
		}

		/// Make a LOC void and provide a replacer.
		#[pallet::weight(T::WeightInfo::make_void_and_replace())]
		pub fn make_void_and_replace(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			#[pallet::compact] replacer_loc_id: T::LocId,
		) -> DispatchResultWithPostInfo {
			Self::do_make_void(origin, loc_id, Some(replacer_loc_id))
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

		fn do_make_void(
			origin: OriginFor<T>,
			loc_id: T::LocId,
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
					if replacer_loc.replacer_of.is_some() {
						Err(Error::<T>::ReplacerLocAlreadyReplacing)?
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

			let loc_void_info = LocVoidInfo {
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

		fn link_with_account(
			account_id: &<T as frame_system::Config>::AccountId,
			loc_id: &<T as Config>::LocId,
		) {
			if <AccountLocsMap<T>>::contains_key(account_id) {
				<AccountLocsMap<T>>::mutate(account_id, |locs| {
					let list = locs.as_mut().unwrap();
					list.push(loc_id.clone());
				});
			} else {
				<AccountLocsMap<T>>::insert(account_id, Vec::from([loc_id.clone()]));
			}
		}

		fn link_with_identity_loc(
			requester_loc_id: &<T as Config>::LocId,
			loc_id: &<T as Config>::LocId,
		) {
			if <IdentityLocLocsMap<T>>::contains_key(requester_loc_id) {
				<IdentityLocLocsMap<T>>::mutate(requester_loc_id, |locs| {
					let list = locs.as_mut().unwrap();
					list.push(loc_id.clone());
				});
			} else {
				<IdentityLocLocsMap<T>>::insert(requester_loc_id, Vec::from([loc_id.clone()]));
			}
		}

		fn build_open_loc(
			who: &T::AccountId,
			requester: &RequesterOf<T>,
			loc_type: LocType,
		) -> LegalOfficerCaseOf<T> {
			LegalOfficerCaseOf::<T> {
				owner: who.clone(),
				requester: requester.clone(),
				metadata: Vec::new(),
				files: Vec::new(),
				closed: false,
				loc_type: loc_type.clone(),
				links: Vec::new(),
				void_info: None,
				replacer_of: None
			}
		}
	}

	pub fn migrate<T: Config>() -> Weight {
		migration::migrate::<T>()
	}
}
