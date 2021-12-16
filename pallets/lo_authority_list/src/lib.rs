#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{EnsureOrigin, Vec};
use frame_system::ensure_signed;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
use frame_system::RawOrigin;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {

		/// The origin which can add a Logion Legal Officer.
		type AddOrigin: EnsureOrigin<Self::Origin>;

		/// The origin which can remove a Logion Legal Officer.
		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// All LOs indexed by their account ID.
	#[pallet::storage]
	#[pallet::getter(fn legal_officer_set)]
	pub type LegalOfficerSet<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, bool>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub legal_officers: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { legal_officers: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_legal_officers(&self.legal_officers);
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(<T as frame_system::Config>::AccountId = "AccountId")]
	pub enum Event<T: Config> {
		/// Issued when an LO is added to the list. [accountId]
		LoAdded(<T as frame_system::Config>::AccountId),
		/// Issued when an LO is removed from the list. [accountId]
		LoRemoved(<T as frame_system::Config>::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The LO is already in the list.
		AlreadyExists,
		/// The LO is not in the list.
		NotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Adds a new LO to the list
		#[pallet::weight(0)]
		pub fn add_legal_officer(
			origin: OriginFor<T>,
			legal_officer_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::AddOrigin::ensure_origin(origin)?;
			if <LegalOfficerSet<T>>::contains_key(&legal_officer_id) {
				Err(Error::<T>::AlreadyExists)?
			} else {
				<LegalOfficerSet<T>>::insert(legal_officer_id.clone(), true);

				Self::deposit_event(Event::LoAdded(legal_officer_id));
				Ok(().into())
			}
		}

		/// Removes a LO from the list
		#[pallet::weight(0)]
		pub fn remove_legal_officer(
			origin: OriginFor<T>,
			legal_officer_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::RemoveOrigin::ensure_origin(origin)?;
			if ! <LegalOfficerSet<T>>::contains_key(&legal_officer_id) {
				Err(Error::<T>::NotFound)?
			} else {
				<LegalOfficerSet<T>>::remove(&legal_officer_id);

				Self::deposit_event(Event::LoRemoved(legal_officer_id));
				Ok(().into())
			}
		}
	}
}

pub type OuterOrigin<T> = <T as frame_system::Config>::Origin;

impl<T: Config> EnsureOrigin<OuterOrigin<T>> for Pallet<T> {
	type Success = T::AccountId;

	fn try_origin(o: OuterOrigin<T>) -> Result<Self::Success, OuterOrigin<T>> {
		let result = ensure_signed(o.clone());
		match result {
			Ok(who) =>
				if ! <LegalOfficerSet<T>>::contains_key(&who) {
					Err(o)
				} else {
					Ok(who.clone())
				}
			Err(_) => Err(o)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> OuterOrigin<T> {
		let first_member = match <LegalOfficerSet<T>>::iter().next() {
			Some(pair) => pair.0.clone(),
			None => Default::default(),
		};
		OuterOrigin::<T>::from(RawOrigin::Signed(first_member.clone()))
	}
}

impl<T: Config> Pallet<T> {
	fn initialize_legal_officers(legal_officers: &Vec<T::AccountId>) {
		for legal_officer in legal_officers {
			LegalOfficerSet::<T>::insert(legal_officer, true);
		}
	}
}
