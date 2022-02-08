use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{assert_ok, traits::EnsureOrigin};
use frame_system::RawOrigin;
use sp_std::{vec, vec::Vec, boxed::Box};

use crate::Pallet as LogionLoc;
use crate::Config;

benchmarks! {
	create_polkadot_identity_loc {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
	}: _(caller, Default::default(), Default::default())

	create_logion_identity_loc {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
	}: _(caller, Default::default())

	create_polkadot_transaction_loc {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
	}: _(caller, Default::default(), Default::default())

	create_logion_transaction_loc {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let identity_loc_id = into_loc_id::<T>(0);
		assert_ok!(LogionLoc::<T>::create_logion_identity_loc(caller.clone().into(), identity_loc_id));
		assert_ok!(LogionLoc::<T>::close(caller.clone().into(), identity_loc_id));
		let loc_id = into_loc_id::<T>(1);
	}: _(caller, loc_id, identity_loc_id)

	add_metadata {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let loc_id = Default::default();
		let item = MetadataItem {
			name: vec![1u8, 2u8, 3u8],
			value: vec![4u8, 5u8, 6u8],
			submitter: Default::default(),
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id, item)

	add_file {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let loc_id = Default::default();
		let file = File {
			hash: Default::default(),
			nature: vec![1u8, 2u8, 3u8],
			submitter: Default::default(),
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id, file)

	add_link {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let linked_loc_id = into_loc_id::<T>(0);
		let link = LocLink {
			id: linked_loc_id.clone(),
			nature: vec![1u8, 2u8, 3u8],
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), linked_loc_id, Default::default()));
		let loc_id = into_loc_id::<T>(1);
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id, link)

	close {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let loc_id = Default::default();
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id)

	make_void {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let loc_id = Default::default();
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id)

	make_void_and_replace {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let replacer_loc_id = into_loc_id::<T>(0);
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), replacer_loc_id, Default::default()));
		let loc_id = into_loc_id::<T>(1);
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(caller.clone().into(), loc_id, Default::default()));
	}: _(caller, loc_id, replacer_loc_id)

	create_collection_loc {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
	}: _(caller, Default::default(), Default::default(), Option::None, Option::Some(1))

	add_collection_item {
		let caller = <T as crate::Config>::CreateOrigin::successful_origin().into().ok().unwrap();
		let loc_id = into_loc_id::<T>(0);
		let requester: T::AccountId = whitelisted_caller();
		assert_ok!(LogionLoc::<T>::create_collection_loc(caller.clone().into(), loc_id, requester.clone(), Option::None, Option::Some(1)));
		assert_ok!(LogionLoc::<T>::close(caller.clone().into(), loc_id));
	}: _(RawOrigin::Signed(requester), loc_id, Default::default(), Default::default())
}

fn into_loc_id<T: pallet::Config>(value: u128) -> <T as crate::Config>::LocId {
	<T as crate::Config>::LocId::decode(&mut &value.encode()[..]).unwrap()
}

impl_benchmark_test_suite!(
	LogionLoc,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
