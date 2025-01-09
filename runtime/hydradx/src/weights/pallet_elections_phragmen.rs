// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-01-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/hydradx
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet
// pallet-elections-phragmen
// --extrinsic
// *
// --heap-pages
// 4096
// --steps
// 50
// --repeat
// 20
// --template=scripts/pallet-weight-template.hbs
// --output
// runtime/hydradx/src/weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_elections_phragmen` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for HydraWeight<T> {
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 10]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 45_347_000 picoseconds.
		Weight::from_parts(45_888_846, 4764)
			// Standard Error: 2_855
			.saturating_add(Weight::from_parts(141_114, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 10]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 59_570_000 picoseconds.
		Weight::from_parts(60_073_597, 4764)
			// Standard Error: 3_867
			.saturating_add(Weight::from_parts(183_927, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 10]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 59_796_000 picoseconds.
		Weight::from_parts(60_444_604, 4764)
			// Standard Error: 3_581
			.saturating_add(Weight::from_parts(154_980, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `761`
		//  Estimated: `4764`
		// Minimum execution time: 57_928_000 picoseconds.
		Weight::from_parts(58_355_000, 4764)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2148 + c * (48 ±0)`
		//  Estimated: `3633 + c * (48 ±0)`
		// Minimum execution time: 43_668_000 picoseconds.
		Weight::from_parts(44_456_066, 3633)
			// Standard Error: 556
			.saturating_add(Weight::from_parts(63_106, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351 + c * (48 ±0)`
		//  Estimated: `1836 + c * (48 ±0)`
		// Minimum execution time: 36_071_000 picoseconds.
		Weight::from_parts(36_935_879, 1836)
			// Standard Error: 400
			.saturating_add(Weight::from_parts(36_494, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2450`
		//  Estimated: `3935`
		// Minimum execution time: 53_483_000 picoseconds.
		Weight::from_parts(54_413_000, 3935)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1491`
		//  Estimated: `2976`
		// Minimum execution time: 38_934_000 picoseconds.
		Weight::from_parts(39_308_000, 2976)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2590`
		//  Estimated: `6196`
		// Minimum execution time: 77_407_000 picoseconds.
		Weight::from_parts(78_636_000, 6196)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Elections::Voting` (r:385 w:384)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:384 w:384)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:384 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:384 w:384)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[384, 768]`.
	/// The range of component `d` is `[0, 384]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + d * (631 ±0) + v * (56 ±0)`
		//  Estimated: `32254 + d * (3774 ±0) + v * (24 ±0)`
		// Minimum execution time: 9_182_000 picoseconds.
		Weight::from_parts(9_678_000, 32254)
			// Standard Error: 8_988
			.saturating_add(Weight::from_parts(198_267, 0).saturating_mul(v.into()))
			// Standard Error: 19_573
			.saturating_add(Weight::from_parts(62_216_315, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 24).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:769 w:0)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:73 w:73)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Elections::ElectionRounds` (r:1 w:1)
	/// Proof: `Elections::ElectionRounds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 100]`.
	/// The range of component `v` is `[1, 768]`.
	/// The range of component `e` is `[768, 7680]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (29 ±0) + e * (27 ±0) + v * (414 ±0)`
		//  Estimated: `278151 + c * (2215 ±6) + e * (13 ±0) + v * (2571 ±3)`
		// Minimum execution time: 3_154_389_000 picoseconds.
		Weight::from_parts(3_158_991_000, 278151)
			// Standard Error: 642_536
			.saturating_add(Weight::from_parts(22_425_704, 0).saturating_mul(v.into()))
			// Standard Error: 67_032
			.saturating_add(Weight::from_parts(1_553_668, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(31_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2215).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 13).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2571).saturating_mul(v.into()))
	}
}