use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use sp_std::{vec, vec::Vec, boxed::Box};

#[allow(unused)]
use crate::Module as LogionLoc;

benchmarks! {
	create_loc {
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), Default::default())
}

impl_benchmark_test_suite!(
	LogionLoc,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
