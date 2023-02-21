// This file is part of HydraDX.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::assert_scheduled_ids;
use crate::tests::mock::*;
use crate::tests::{empty_vec, set_storage_bond_config, ScheduleBuilder};
use crate::{Error, Event, Order, PoolType, Schedule, ScheduleId, Trade};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::BlockNumberFor;
use pretty_assertions::assert_eq;
use sp_runtime::traits::ConstU32;
use sp_runtime::DispatchError;
use sp_runtime::DispatchError::BadOrigin;
use sp_runtime::{BoundedVec, FixedU128};
use std::ops::RangeInclusive;
pub type Price = FixedU128;
use crate::reserve_identifier;
use orml_traits::MultiReservableCurrency;
use orml_traits::NamedMultiReservableCurrency;
use test_case::test_case;

#[test]
fn schedule_should_reserve_all_total_amount_as_named_reserve() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange

			let total_amount = 100 * ONE;
			let schedule = ScheduleBuilder::new()
				.with_total_amount(total_amount)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();
			//Act
			set_block_number(500);
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			let schedule_id = 1;

			let named_reserve_id = reserve_identifier(schedule_id);

			assert_eq!(
				total_amount,
				Currencies::reserved_balance_named(&named_reserve_id, HDX.into(), &ALICE.into())
			);
		});
}

#[test]
fn schedule_should_store_schedule_for_next_block_when_no_blocknumber_specified() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();

			//Act
			set_block_number(500);
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			let schedule_id = 1;
			let stored_schedule = DCA::schedules(schedule_id).unwrap();
			assert_eq!(stored_schedule, ScheduleBuilder::new().build());

			//Check if schedule ids are stored
			let schedule_ids = DCA::schedule_ids_per_block(501);
			assert!(DCA::schedule_ids_per_block(501).is_some());
			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1]);
			assert_eq!(schedule_ids.unwrap(), expected_scheduled_ids_for_next_block);

			//Check if schedule ownership is created
			assert!(DCA::owner_of(schedule_id).is_some());
			assert_eq!(DCA::owner_of(schedule_id).unwrap(), ALICE);
		});
}

#[test]
fn schedule_should_work_when_multiple_schedules_stored() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();

			//Act
			set_block_number(500);

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule.clone(), Option::None));
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			assert!(DCA::schedules(1).is_some());
			assert!(DCA::schedules(2).is_some());

			let scheduled_ids_for_next_block = DCA::schedule_ids_per_block(501).unwrap();

			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1, 2]);
			assert_eq!(scheduled_ids_for_next_block, expected_scheduled_ids_for_next_block);
		});
}

#[test]
fn schedule_should_work_when_block_is_specified_by_user() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();

			//Act
			set_block_number(500);
			assert_ok!(DCA::schedule(
				Origin::signed(ALICE),
				schedule.clone(),
				Option::Some(600)
			));

			//Assert
			let schedule_id = 1;
			let stored_schedule = DCA::schedules(schedule_id).unwrap();
			assert_eq!(stored_schedule, schedule);

			//Check if schedule ids are stored
			let schedule_ids = DCA::schedule_ids_per_block(600);
			assert!(DCA::schedule_ids_per_block(600).is_some());
			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1]);
			assert_eq!(schedule_ids.unwrap(), expected_scheduled_ids_for_next_block);

			//Check if schedule ownership is created
			assert!(DCA::owner_of(schedule_id).is_some());
			assert_eq!(DCA::owner_of(schedule_id).unwrap(), ALICE);
		});
}

#[test]
fn schedule_should_emit_necessary_events() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();

			//Act
			set_block_number(500);
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			expect_events(vec![
				Event::Scheduled { id: 1, who: ALICE }.into(),
				Event::ExecutionPlanned {
					id: 1,
					who: ALICE,
					block: 501,
				}
				.into(),
			]);
		});
}

#[test]
fn schedule_should_emit_necessary_events_when_multiple_schedules_are_created() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();
			let schedule2 = ScheduleBuilder::new().build();

			//Act and assert
			set_block_number(500);

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			expect_events(vec![
				Event::Scheduled { id: 1, who: ALICE }.into(),
				Event::ExecutionPlanned {
					id: 1,
					who: ALICE,
					block: 501,
				}
				.into(),
			]);

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule2, Option::Some(1000)));
			expect_events(vec![
				Event::Scheduled { id: 2, who: ALICE }.into(),
				Event::ExecutionPlanned {
					id: 2,
					who: ALICE,
					block: 1000,
				}
				.into(),
			]);
		});
}

#[test]
fn schedule_should_throw_error_when_user_has_not_enough_balance() {
	let total_amount_to_be_taken = 100 * ONE;
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, total_amount_to_be_taken - 1)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().with_total_amount(100 * ONE).build();

			//Act
			set_block_number(500);
			assert_noop!(
				DCA::schedule(Origin::signed(ALICE), schedule, Option::None),
				Error::<Test>::InsufficientBalanceForTotalAmount
			);
		});
}

#[test]
fn schedule_should_fail_when_not_called_by_user() {
	ExtBuilder::default().build().execute_with(|| {
		//Arrange
		let schedule = ScheduleBuilder::new().build();

		//Act and assert
		assert_noop!(DCA::schedule(Origin::none(), schedule, Option::None), BadOrigin);
	});
}

#[test_case(1)]
#[test_case(499)]
#[test_case(500)]
fn schedule_should_fail_when_specified_next_block_is_not_greater_than_current_block(block: BlockNumberFor<Test>) {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new().build();
			set_block_number(500);

			//Act and assert
			assert_noop!(
				DCA::schedule(Origin::signed(ALICE), schedule, Option::Some(block)),
				Error::<Test>::BlockNumberIsNotInFuture
			);
		});
}

#[test]
fn schedule_should_schedule_for_consequent_block_when_next_block_is_full() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 1000000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			set_block_number(500);

			for _ in RangeInclusive::new(1, 20) {
				let schedule = ScheduleBuilder::new().build();
				assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			}

			//Act
			let schedule = ScheduleBuilder::new().build();
			let schedule_id = 21;
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			let actual_schedule_ids = DCA::schedule_ids_per_block(501).unwrap();
			assert_eq!(20, actual_schedule_ids.len());

			assert_scheduled_ids!(502, vec![schedule_id]);
		});
}

#[test]
fn schedule_should_schedule_for_after_consequent_block_when_both_next_block_and_consquent_block_is_full() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 1000000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange
			set_block_number(500);

			for _ in RangeInclusive::new(1, 40) {
				let schedule = ScheduleBuilder::new().build();
				assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			}

			//Act
			let schedule = ScheduleBuilder::new().build();
			let schedule_id = 41;
			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Assert
			let actual_schedule_ids = DCA::schedule_ids_per_block(501).unwrap();
			assert_eq!(20, actual_schedule_ids.len());

			let actual_schedule_ids = DCA::schedule_ids_per_block(502).unwrap();
			assert_eq!(20, actual_schedule_ids.len());

			assert_scheduled_ids!(503, vec![schedule_id]);
		});
}

#[test]
fn schedule_should_fail_when_total_amount_is_smaller_than_storage_bond_and_sold_currency_is_native() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE)])
		.build()
		.execute_with(|| {
			//Arrange

			let schedule = ScheduleBuilder::new()
				.with_total_amount(*OriginalStorageBondInNative)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			//Act and Assert
			set_block_number(500);

			assert_noop!(
				DCA::schedule(Origin::signed(ALICE), schedule, Option::None),
				Error::<Test>::TotalAmountShouldBeLargerThanStorageBond
			);
		});
}

#[test]
fn schedule_should_pass_when_total_amount_in_non_native_currency_is_bigger_than_storage_bond_in_native() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE), (ALICE, DAI, 10000 * ONE)])
		.with_initial_pool(FixedU128::from(1), FixedU128::from_float(0.5))
		.build()
		.execute_with(|| {
			//Arrange

			let schedule = ScheduleBuilder::new()
				.with_total_amount(*OriginalStorageBondInNative * 9 / 10)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: HDX,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			//Act and Assert
			set_block_number(500);

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
		});
}

#[test]
fn schedule_should_fail_when_total_amount_in_non_native_currency_is_smaller_than_storage_bond_in_native() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, HDX, 10000 * ONE), (ALICE, DAI, 10000 * ONE)])
		.with_initial_pool(FixedU128::from(1), FixedU128::from_float(0.5))
		.build()
		.execute_with(|| {
			//Arrange

			let schedule = ScheduleBuilder::new()
				.with_total_amount(*OriginalStorageBondInNative / 3)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: HDX,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			//Act and Assert
			set_block_number(500);

			assert_noop!(
				DCA::schedule(Origin::signed(ALICE), schedule, Option::None),
				Error::<Test>::TotalAmountShouldBeLargerThanStorageBond
			);
		});
}

fn create_bounded_vec_with_schedule_ids(schedule_ids: Vec<ScheduleId>) -> BoundedVec<ScheduleId, ConstU32<5>> {
	let bounded_vec: BoundedVec<ScheduleId, sp_runtime::traits::ConstU32<5>> = schedule_ids.try_into().unwrap();
	bounded_vec
}

pub fn set_block_number(n: u64) {
	System::set_block_number(n);
}
