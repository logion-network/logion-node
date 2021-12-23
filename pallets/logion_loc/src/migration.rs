use frame_support::codec::{Decode, Encode};
use frame_support::debug;
use frame_support::traits::Get;
use frame_support::traits::Vec;
use frame_support::weights::Weight;

use crate::{Config, File, LegalOfficerCaseOf, LocLink, LocMap, LocType, MetadataItem, pallet, PalletStorageVersion, StorageVersion};

pub fn migrate<T: Config>() -> Weight {
	do_migrate::<T, _>(StorageVersion::V2MakeLocVoid, v2::migrate::<T>)
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

mod v2 {
	use crate::LocVoidInfo;
	use super::*;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
	struct LegalOfficerCaseV2<AccountId, Hash, LocId> {
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

	type LegalOfficerCaseOfV2<T> = LegalOfficerCaseV2<<T as frame_system::Config>::AccountId, <T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

	pub(crate) fn migrate<T: Config>() -> Weight {
		<LocMap<T>>::translate::<LegalOfficerCaseOfV2<T>, _>(
			|loc_id: T::LocId, loc: LegalOfficerCaseOfV2<T>| {
				debug::info!("Migrating LOC: {:?}", loc_id);
				debug::info!("From: {:?}", loc);
				let new_loc = LegalOfficerCaseOf::<T> {
					owner: loc.owner.clone(),
					requester: crate::Requester::Account(loc.requester.clone()),
					metadata: loc.metadata.clone(),
					files: loc.files.clone(),
					closed: loc.closed.clone(),
					loc_type: loc.loc_type.clone(),
					links: loc.links.clone(),
					void_info: loc.void_info.clone(),
					replacer_of: loc.replacer_of.clone(),
				};
				debug::info!("To: {:?}", new_loc);
				Some(new_loc)
			}
		);
		let count = <LocMap<T>>::iter().count();
		T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
	}
}
