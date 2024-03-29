
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gerard-XPS-13-9305`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/logion-node
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --pallet
// pallet_multisig
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_586_000 picoseconds.
		Weight::from_parts(9_097_976, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(440, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235 + s * (4 ±0)`
		//  Estimated: `4250`
		// Minimum execution time: 36_048_000 picoseconds.
		Weight::from_parts(34_727_113, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			// Standard Error: 4_438
			.saturating_add(Weight::from_parts(146_412, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_155, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `4250`
		// Minimum execution time: 20_014_000 picoseconds.
		Weight::from_parts(19_238_282, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			// Standard Error: 3_709
			.saturating_add(Weight::from_parts(82_658, 0).saturating_mul(s.into()))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(1_151, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377 + s * (36 ±0)`
		//  Estimated: `4250`
		// Minimum execution time: 39_548_000 picoseconds.
		Weight::from_parts(37_731_312, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			// Standard Error: 37_762
			.saturating_add(Weight::from_parts(342_317, 0).saturating_mul(s.into()))
			// Standard Error: 70
			.saturating_add(Weight::from_parts(1_088, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235 + s * (4 ±0)`
		//  Estimated: `4250`
		// Minimum execution time: 33_669_000 picoseconds.
		Weight::from_parts(34_694_225, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			// Standard Error: 4_266
			.saturating_add(Weight::from_parts(141_583, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_approve(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `4250`
		// Minimum execution time: 16_782_000 picoseconds.
		Weight::from_parts(22_205_158, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(785), added: 3260, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `441 + s * (4 ±0)`
		//  Estimated: `4250`
		// Minimum execution time: 33_701_000 picoseconds.
		Weight::from_parts(36_858_745, 0)
			.saturating_add(Weight::from_parts(0, 4250))
			// Standard Error: 5_473
			.saturating_add(Weight::from_parts(64_461, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
