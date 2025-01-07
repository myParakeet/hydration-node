// Copyright (C) 2020-2024  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use frame_support::{
	traits::OnRuntimeUpgrade,
	weights::Weight,
};

pub struct OnRuntimeUpgradeMigration;
use super::Runtime;

impl cumulus_pallet_xcmp_queue::migration::v5::V5Config for Runtime {
	type ChannelList = ParachainSystem;
}

impl OnRuntimeUpgrade for OnRuntimeUpgradeMigration {
	fn on_runtime_upgrade() -> Weight {
		cumulus_pallet_xcmp_queue::migration::v5::MigrateV4ToV5::<Runtime>::on_runtime_upgrade()
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		cumulus_pallet_xcmp_queue::migration::v5::MigrateV4ToV5::<Runtime>::post_upgrade(state)
	}
}
