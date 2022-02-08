use frame_support::codec::{Decode, Encode};
use frame_support::debug;
use frame_support::traits::Get;
use frame_support::traits::Vec;
use frame_support::weights::Weight;

use crate::{Config, File, LegalOfficerCaseOf, LocLink, LocMap, LocType, MetadataItem, pallet, PalletStorageVersion, StorageVersion};

pub fn migrate<T: Config>() -> Weight {
	do_migrate::<T, _>(StorageVersion::V4ItemSubmitter, v4::migrate::<T>)
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

mod v4 {
	use crate::{LocVoidInfo, Requester};

	use super::*;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
	struct LegalOfficerCaseV4<AccountId, Hash, LocId> {
		owner: AccountId,
		requester: Requester<AccountId, LocId>,
		metadata: Vec<MetadataItem<AccountId>>,
		files: Vec<File<Hash, AccountId>>,
		closed: bool,
		loc_type: LocType,
		links: Vec<LocLink<LocId>>,
		void_info: Option<LocVoidInfo<LocId>>,
		replacer_of: Option<LocId>,
	}

	type LegalOfficerCaseOfV4<T> = LegalOfficerCaseV4<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

	pub(crate) fn migrate<T: Config>() -> Weight {
		<LocMap<T>>::translate::<LegalOfficerCaseOfV4<T>, _>(
			|loc_id: T::LocId, loc: LegalOfficerCaseOfV4<T>| {
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
					void_info: loc.void_info.clone(),
					replacer_of: loc.replacer_of.clone(),
					collection_last_block_submission: Option::None,
					collection_max_size: Option::None,
				};
				debug::info!("To: {:?}", new_loc);
				Some(new_loc)
			}
		);
		let count = <LocMap<T>>::iter().count();
		T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
	}
}
