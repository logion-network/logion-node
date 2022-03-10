#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*
	};
	use logion_shared::{MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory, IsLegalOfficer};
	use frame_support::traits::{Vec, UnfilteredDispatchable};
	use pallet_multisig::{WeightInfo, Timepoint, OpaqueCall};

	#[pallet::config]
	pub trait Config: frame_system::Config {

		/// Implementation of multisig "approve as multi"
		type MultisigApproveAsMultiCallFactory: MultisigApproveAsMultiCallFactory<Self::Origin, Self::AccountId, Timepoint<Self::BlockNumber>>;

		/// Implementation of multisig "as multi"
		type MultisigAsMultiCallFactory: MultisigAsMultiCallFactory<Self::Origin, Self::AccountId, Timepoint<Self::BlockNumber>>;

		/// Query for checking that a signer is a legal officer
		type IsLegalOfficer: IsLegalOfficer<Self::AccountId>;

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
		/// The set of signatories is invalid (size <> from 2 or does not contain only legal officers on transfer creation).
		InvalidSignatories,
		/// The transfer initiator is a legal officer.
		WrongInitiator,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Create a vault transfer. The creator must not be a legal officer.
		#[pallet::weight({
			// Weight computation comes from multisig pallet
			let s = legal_officers.len() as u32;

			T::WeightInfo::approve_as_multi_create(s)
			.max(T::WeightInfo::approve_as_multi_approve(s))
			.max(T::WeightInfo::approve_as_multi_complete(s))
			.saturating_add(*max_weight)
		})]
		pub fn request_call(
			origin: OriginFor<T>,
			legal_officers: Vec<T::AccountId>,
			call_hash: [u8; 32],
			max_weight: Weight,
		) -> DispatchResultWithPostInfo {
			if legal_officers.len() != 2
					|| !T::IsLegalOfficer::is_legal_officer(&legal_officers[0])
					|| !T::IsLegalOfficer::is_legal_officer(&legal_officers[1]) {
				Err(Error::<T>::InvalidSignatories)?
			} else {
				let who = ensure_signed(origin.clone())?;
				if T::IsLegalOfficer::is_legal_officer(&who) {
					Err(Error::<T>::WrongInitiator)?
				} else {
					Self::dispatch_create_multi(origin, legal_officers, call_hash, max_weight)
				}
			}
		}

		/// Approves a vault transfer.
		#[pallet::weight({
			// Weight computation comes from multisig pallet
			let s = other_signatories.len() as u32;
			let z = call.len() as u32;

			T::WeightInfo::as_multi_create(s, z)
			.max(T::WeightInfo::as_multi_create_store(s, z))
			.max(T::WeightInfo::as_multi_approve(s, z))
			.max(T::WeightInfo::as_multi_complete(s, z))
			.saturating_add(*max_weight)
		})]
		pub fn approve_call(
			origin: OriginFor<T>,
			other_signatories: Vec<T::AccountId>,
			call: OpaqueCall,
			timepoint: Timepoint<T::BlockNumber>,
			max_weight: Weight,
		) -> DispatchResultWithPostInfo {
			if other_signatories.len() != 2 {
				Err(Error::<T>::InvalidSignatories)?
			} else {
				Self::dispatch_as_multi(origin, other_signatories, call, timepoint, max_weight)
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn dispatch_create_multi(
			origin: OriginFor<T>,
			legal_officers: Vec<T::AccountId>,
			call_hash: [u8; 32],
			max_weight: Weight,
		) -> DispatchResultWithPostInfo {
			let call = <T as Config>::MultisigApproveAsMultiCallFactory::build_approve_as_multi_call(
					2,
					legal_officers,
					None,
					call_hash,
					max_weight
			);
			call.dispatch_bypass_filter(origin)
		}

		fn dispatch_as_multi(
			origin: OriginFor<T>,
			other_signatories: Vec<T::AccountId>,
			call: OpaqueCall,
			timepoint: Timepoint<T::BlockNumber>,
			max_weight: Weight,
		) -> DispatchResultWithPostInfo {
			let call = <T as Config>::MultisigAsMultiCallFactory::build_as_multi_call(
					2,
					other_signatories,
					Some(timepoint),
					call,
					false,
					max_weight
			);
			call.dispatch_bypass_filter(origin)
		}
	}
}
