
//! Autogenerated weights for `pallet_sudo`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_sudo
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// runtime/src/weights/pallet_sudo.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_sudo`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sudo::WeightInfo for WeightInfo<T> {
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn set_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 9_990_000 picoseconds.
		Weight::from_parts(10_314_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 10_645_000 picoseconds.
		Weight::from_parts(11_178_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:0)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn sudo_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 10_803_000 picoseconds.
		Weight::from_parts(11_243_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Sudo::Key` (r:1 w:1)
	/// Proof: `Sudo::Key` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn remove_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132`
		//  Estimated: `1517`
		// Minimum execution time: 8_691_000 picoseconds.
		Weight::from_parts(9_060_000, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
