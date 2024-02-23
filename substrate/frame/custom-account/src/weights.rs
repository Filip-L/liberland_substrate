
//! Autogenerated weights for pallet_custom_account
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-01, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `kacper-HP-ProBook-445-G7`, CPU: `AMD Ryzen 7 4700U with Radeon Graphics`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/substrate
// benchmark
// pallet
// --pallet=pallet_custom_account
// --execution=wasm
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=frame/custom-account/src/weights.rs
// --extrinsic=*
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_custom_account.
pub trait WeightInfo {
	fn execute() -> Weight;
}

/// Weights for pallet_custom_account using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 22_643_000 picoseconds.
		Weight::from_parts(23_544_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 22_643_000 picoseconds.
		Weight::from_parts(23_544_000, 0)
	}
}
