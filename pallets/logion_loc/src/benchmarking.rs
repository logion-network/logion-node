use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use frame_support::assert_ok;
use sp_std::{vec, vec::Vec, boxed::Box};

use crate::Pallet as LogionLoc;

benchmarks! {
	create_loc {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), Default::default(), Default::default())

	add_metadata {
		let caller: T::AccountId = whitelisted_caller();
		let loc_id = Default::default();
		let item = MetadataItem {
			name: vec![1u8, 2u8, 3u8],
			value: vec![4u8, 5u8, 6u8],
		};
		assert_ok!(LogionLoc::<T>::create_loc(RawOrigin::Signed(caller.clone()).into(), loc_id, Default::default()));
	}: _(RawOrigin::Signed(caller), loc_id, item)

	add_hash {
		let caller: T::AccountId = whitelisted_caller();
		let loc_id = Default::default();
		let hash = Default::default();
		assert_ok!(LogionLoc::<T>::create_loc(RawOrigin::Signed(caller.clone()).into(), loc_id, Default::default()));
	}: _(RawOrigin::Signed(caller), loc_id, hash)

	close {
		let caller: T::AccountId = whitelisted_caller();
		let loc_id = Default::default();
		assert_ok!(LogionLoc::<T>::create_loc(RawOrigin::Signed(caller.clone()).into(), loc_id, Default::default()));
	}: _(RawOrigin::Signed(caller), loc_id)
}

impl_benchmark_test_suite!(
	LogionLoc,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
