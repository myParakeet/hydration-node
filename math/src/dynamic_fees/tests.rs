use super::types::*;
use crate::dynamic_fees::{recalculate_asset_fee, recalculate_protocol_fee, compute_dynamic_fee};
use num_traits::Zero;
use sp_arithmetic::{FixedU128, Permill};

#[test]
fn asset_fee_should_decrease_when_in_is_greater_than_out() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 1;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::zero(),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(9);

	let calculated_fee = recalculate_asset_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn asset_fee_should_increase_when_out_is_greater_than_int() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 5,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 1;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::zero(),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(13);

	let calculated_fee = recalculate_asset_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn protocol_fee_should_decrease_when_out_is_greater_than_int() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 5,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 1;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::zero(),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(7);

	let calculated_fee = recalculate_protocol_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn protocol_fee_should_increase_when_in_is_greater_than_out() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 1;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::zero(),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(11);

	let calculated_fee = recalculate_protocol_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn protocol_fee_should_decay_when_block_diff_is_greater_than_one() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_float(0.107);

	let calculated_fee = recalculate_protocol_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn asset_fee_should_decay_when_block_diff_is_greater_than_one() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 1000,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_float(0.087);

	let calculated_fee = recalculate_asset_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn asset_fee_should_clamp_to_max_fee() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 5,
		amount_out: 20,
		liquidity: 100,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(30);

	let calculated_fee = recalculate_asset_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn asset_fee_should_clamp_to_min_fee() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 100,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(1);

	let calculated_fee = recalculate_asset_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn procotol_fee_should_clamp_to_min_fee() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 5,
		amount_out: 20,
		liquidity: 100,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(30),
	};

	let expected_fee = Permill::from_percent(1);

	let calculated_fee = recalculate_protocol_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}

#[test]
fn protocol_fee_should_clamp_to_max_fee() {
	// Test parameters
	let volume = OracleEntry {
		amount_in: 25,
		amount_out: 20,
		liquidity: 100,
	};
	let previous_fee = Permill::from_percent(10);
	let last_block_diff = 3;
	let params = FeeParams {
		amplification: FixedU128::from(2),
		decay: FixedU128::from_float(0.001),
		min_fee: Permill::from_percent(1),
		max_fee: Permill::from_percent(15),
	};

	let expected_fee = Permill::from_percent(15);

	let calculated_fee = recalculate_protocol_fee(volume, previous_fee, last_block_diff, params);
	assert_eq!(calculated_fee, expected_fee);
}


#[test]
fn compute_asset_fee_should_decrease_when_in_is_greater_than_out() {
	// Test parameters
	let volume = OracleEntry {
		amount_out: 390_982798935286,
		amount_in: 0,
		liquidity: 200_000_000_000_000_000,
	};
	let current_liquidity = 198045_086_005_323_500;
	let previous_fee = Permill::from_float(0.0025);
	let last_block_diff = 1;
	let params = FeeParams {
		amplification: FixedU128::from(10),
		decay: FixedU128::from_rational(2,10),       // 0.001%
		min_fee: Permill::from_float(0.001),
		max_fee: Permill::from_percent(30),
	};
	dbg!(&previous_fee);

	let expected_fee = Permill::from_percent(9);

	let calculated_asset_fee = compute_dynamic_fee(volume.clone(), current_liquidity, params.clone(), previous_fee, last_block_diff, NetVolumeDirection::OutIn);
	//let calculated_protocol_fee = compute_dynamic_fee(volume, current_liquidity, params, previous_fee, last_block_diff, NetVolumeDirection::InOut);
	dbg!(calculated_asset_fee);
	//dbg!(calculated_protocol_fee);
}
