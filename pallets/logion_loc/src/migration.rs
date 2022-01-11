use frame_support::codec::{Decode, Encode};
use frame_support::debug;
use frame_support::traits::Get;
use frame_support::traits::Vec;
use frame_support::weights::Weight;

use crate::{Config, File, LegalOfficerCaseOf, LocLink, LocMap, LocType, MetadataItem, pallet, PalletStorageVersion, StorageVersion};

pub fn migrate<T: Config>() -> Weight {
	do_migrate::<T, _>(StorageVersion::V3RequesterEnum, v3::migrate::<T>)
}

fn do_migrate<T: Config, F>(from: StorageVersion, migration_fn: F) -> Weight
	where F: FnOnce() -> Weight {
	debug::RuntimeLogger::init();
	let stored_version = <PalletStorageVersion<T>>::try_get();
	let to: StorageVersion = Default::default();
	if stored_version.is_err() || stored_version.unwrap() == from {
		debug::info!("Starting to migrate from {:?} to {:?}", from, &to);
		let weight = migration_fn();
		<PalletStorageVersion<T>>::put(&to);
		debug::info!("Migration ended.");
		weight
	} else {
		debug::info!("The migration {:?} to {:?} was already applied.", from, &to);
		0
	}
}

mod v3 {
	use crate::{LocVoidInfo, Requester};

	use super::*;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
	pub struct MetadataItemV3 {
		name: Vec<u8>,
		value: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
	pub struct FileV3<Hash> {
		hash: Hash,
		nature: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
	struct LegalOfficerCaseV3<AccountId, Hash, LocId> {
		owner: AccountId,
		requester: Requester<AccountId, LocId>,
		metadata: Vec<MetadataItemV3>,
		files: Vec<FileV3<Hash>>,
		closed: bool,
		loc_type: LocType,
		links: Vec<LocLink<LocId>>,
		void_info: Option<LocVoidInfo<LocId>>,
		replacer_of: Option<LocId>
	}

	type LegalOfficerCaseOfV3<T> = LegalOfficerCaseV3<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

	pub(crate) fn migrate<T: Config>() -> Weight {
		<LocMap<T>>::translate::<LegalOfficerCaseOfV3<T>, _>(
			|loc_id: T::LocId, loc: LegalOfficerCaseOfV3<T>| {
				debug::info!("Migrating LOC: {:?}", loc_id);
				debug::info!("From: {:?}", loc);
				let mut new_loc = LegalOfficerCaseOf::<T> {
					owner: loc.owner.clone(),
					requester: loc.requester.clone(),
					metadata: Vec::new(),
					files: Vec::new(),
					closed: loc.closed.clone(),
					loc_type: loc.loc_type.clone(),
					links: loc.links.clone(),
					void_info: loc.void_info.clone(),
					replacer_of: loc.replacer_of.clone(),
				};
				new_loc.metadata.extend(loc.metadata.iter().map(|item| {
					MetadataItem::<<T as frame_system::Config>::AccountId> {
						name: item.name.clone(),
						value: item.value.clone(),
						submitter: loc.owner.clone(),
					}
				}));
				new_loc.files.extend(loc.files.iter().map(|item| {
					File::<<T as pallet::Config>::Hash, <T as frame_system::Config>::AccountId> {
						hash: item.hash.clone(),
						nature: item.nature.clone(),
						submitter: loc.owner.clone(),
					}
				}));
				debug::info!("To: {:?}", new_loc);
				Some(new_loc)
			}
		);
		let count = <LocMap<T>>::iter().count();
		T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
	}
}
