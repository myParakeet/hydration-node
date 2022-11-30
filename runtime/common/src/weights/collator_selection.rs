// This file is part of Hydra-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// --chain=local
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_collator_selection
// --output=collator_selection.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_collator_selection::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn set_invulnerables(b: u32) -> Weight {
		Weight::from_ref_time(12_265_000 as u64) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(87_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_desired_candidates() -> Weight {
		Weight::from_ref_time(15_221_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_candidacy_bond() -> Weight {
		Weight::from_ref_time(11_762_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn register_as_candidate(c: u32) -> Weight {
		Weight::from_ref_time(47_945_000 as u64) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(459_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn leave_intent(c: u32) -> Weight {
		Weight::from_ref_time(34_269_000 as u64) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(488_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn note_author() -> Weight {
		Weight::from_ref_time(43_846_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn new_session(r: u32, c: u32) -> Weight {
		Weight::zero()
			.saturating_add(Weight::from_ref_time(4_049_000 as u64).saturating_mul(r as u64)) // Standard Error: 1_694_000
			.saturating_add(Weight::from_ref_time(18_735_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
