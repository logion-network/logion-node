#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use logion_shared::{CreateRecoveryCallFactory, LocQuery};
	use frame_support::traits::{Vec, UnfilteredDispatchable};
	pub use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {

		/// Implementation of recovery config creation
		type CreateRecoveryCallFactory: CreateRecoveryCallFactory<Self::Origin, Self::AccountId, Self::BlockNumber>;

		/// Query for checking the existence of a closed Identity LOC
		type LocQuery: LocQuery<Self::AccountId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::event]
	pub enum Event<T: Config> {
		
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The set of legal officers is invalid (size <> from 2).
		InvalidLegalOfficers,
		/// One or both legal officers in the friends list did not yet close an Identity LOC for the account.
		MissingIdentityLoc,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Create a recovery configuration for your account. The legal officers must all have closed their Identity LOC.
		#[pallet::weight(T::WeightInfo::create_recovery())]
		pub fn create_recovery(
			origin: OriginFor<T>,
			legal_officers: Vec<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			if legal_officers.len() != 2 {
				Err(Error::<T>::InvalidLegalOfficers)?
			} else {
				let who = ensure_signed(origin.clone())?;
				if T::LocQuery::has_closed_identity_locs(&who, &legal_officers) {
					Self::dispatch_create_recovery(origin, legal_officers)
				} else {
					Err(Error::<T>::MissingIdentityLoc)?
				}
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn dispatch_create_recovery(origin: OriginFor<T>, legal_officers: Vec<T::AccountId>) -> DispatchResultWithPostInfo {
			let call = <T as Config>::CreateRecoveryCallFactory::build_create_recovery_call(
					legal_officers,
					1,
					<T as frame_system::Config>::BlockNumber::from(0u32)
			);
			call.dispatch_bypass_filter(origin)
		}
	}
}
