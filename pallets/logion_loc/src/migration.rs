use frame_support::codec::{Decode, Encode};
use frame_support::debug;
use frame_support::traits::Get;
use frame_support::traits::Vec;
use frame_support::weights::Weight;

use crate::{Config, File, LegalOfficerCaseOf, LocLink, LocMap, LocType, MetadataItem, pallet, PalletStorageVersion, StorageVersion};

pub fn migrate<T: Config>() -> Weight {
	do_migrate::<T, _>(StorageVersion::V1, StorageVersion::V2MakeLocVoid, v1::migrate::<T>)
}

fn do_migrate<T: Config, F>(from: StorageVersion, to:StorageVersion, migration_fn: F) -> Weight
	where F: FnOnce() -> Weight {
	debug::RuntimeLogger::init();
	if <PalletStorageVersion<T>>::get() == from {
		debug::info!("Starting to migrate from {:?} to {:?}", from, to);
		let weight = migration_fn();
		<PalletStorageVersion<T>>::put(to);
		debug::info!("Migration ended.");
		weight
	} else {
		debug::info!("No Migrating needed.");
		0
	}
}

mod v1 {
	use super::*;

	#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
	struct LegalOfficerCaseV1<AccountId, Hash, LocId> {
		owner: AccountId,
		requester: AccountId,
		metadata: Vec<MetadataItem>,
		files: Vec<File<Hash>>,
		closed: bool,
		loc_type: LocType,
		links: Vec<LocLink<LocId>>,
	}

	type LegalOfficerCaseOfV1<T> = LegalOfficerCaseV1<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

	pub(crate) fn migrate<T: Config>() -> Weight {
		<LocMap<T>>::translate::<LegalOfficerCaseOfV1<T>, _>(
			|loc_id: T::LocId, loc: LegalOfficerCaseOfV1<T>| {
				debug::info!("Migrating LOC: {:?}", loc_id);
				debug::info!("From: {:?}", loc);
				let new_loc = LegalOfficerCaseOf::<T> {
					owner: loc.owner.clone(),
					requester: loc.requester.clone(),
					metadata: loc.metadata.clone(),
					files: loc.files.clone(),
					closed: loc.closed.clone(),
					loc_type: loc.loc_type.clone(),
					links: loc.links.clone(),
					void_info: None,
					replacer_of: None,
				};
				debug::info!("To: {:?}", new_loc);
				Some(new_loc)
			}
		);
		let count = <LocMap<T>>::iter().count();
		T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
	}
}
