
//! Autogenerated weights for `pallet_bulk`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sulijia-PC`, CPU: `AMD Ryzen 7 5800U with Radeon Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/parachain-magnet-node
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_bulk
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bulk`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bulk::WeightInfo for WeightInfo<T> {
	/// Storage: `BulkPallet::RecordIndex` (r:1 w:1)
	/// Proof: `BulkPallet::RecordIndex` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BulkPallet::BulkRecords` (r:0 w:1)
	/// Proof: `BulkPallet::BulkRecords` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn create_record(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `1489`
		// Minimum execution time: 30_127_000 picoseconds.
		Weight::from_parts(33_526_304, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 1_136
			.saturating_add(Weight::from_parts(2_315, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BulkPallet::RpcUrl` (r:0 w:1)
	/// Proof: `BulkPallet::RpcUrl` (`max_values`: Some(1), `max_size`: Some(302), added: 797, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_rpc_url(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_729_000 picoseconds.
		Weight::from_parts(5_325_688, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
