use crate::tests::*;

/// Auto-generated test
/// This test scenario is generated by python's omnipool implementation - initial implementation done during research.
/// Its purpose is to verify that python and rust omnipool implementation produce same results.
#[test]
fn scenario_05() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), 0, NATIVE_AMOUNT),
			(Omnipool::protocol_account(), 2, 1000 * ONE),
			(LP1, 100, 5000000000000000),
			(LP1, 200, 5000000000000000),
			(LP2, 100, 1000000000000000),
			(LP3, 100, 1000000000000000),
		])
		.with_registered_asset(100)
		.with_registered_asset(200)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.with_token(100, FixedU128::from_float(0.65), LP1, 2000 * ONE)
		.with_token(200, FixedU128::from_float(0.65), LP1, 2000 * ONE)
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_liquidity(
				RuntimeOrigin::signed(LP2),
				100,
				400000000000000
			));

			assert_ok!(Omnipool::sell(
				RuntimeOrigin::signed(LP3),
				100,
				200,
				50000000000000,
				10000000000000
			));

			assert_balance_approx!(Omnipool::protocol_account(), 0, NATIVE_AMOUNT, 10);
			assert_balance_approx!(Omnipool::protocol_account(), 2, 1000000000000000u128, 10);
			assert_balance!(Omnipool::protocol_account(), 1, 13360000000000000);
			assert_balance!(Omnipool::protocol_account(), 100, 2450000000000000);
			assert_balance!(Omnipool::protocol_account(), 200, 1952191235059762);
			assert_balance!(LP1, 100, 3000000000000000);
			assert_balance!(LP1, 200, 3000000000000000);
			assert_balance!(LP2, 100, 600000000000000);
			assert_balance!(LP3, 100, 950000000000000);
			assert_balance!(LP3, 1, 0);
			assert_balance!(LP3, 200, 47808764940238);

			assert_asset_state!(
				2,
				AssetReserveState {
					reserve: 1000000000000000,
					hub_reserve: 500000000000000,
					shares: 1000000000000000,
					protocol_shares: 0,
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				0,
				AssetReserveState {
					reserve: 10000000000000000,
					hub_reserve: 10000000000000000,
					shares: 10000000000000000,
					protocol_shares: 0,
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				100,
				AssetReserveState {
					reserve: 2450000000000000,
					hub_reserve: 1528163265306123,
					shares: 2400000000000000,
					protocol_shares: Balance::zero(),
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				200,
				AssetReserveState {
					reserve: 1952191235059762,
					hub_reserve: 1331836734693877,
					shares: 2000000000000000,
					protocol_shares: Balance::zero(),
					cap: DEFAULT_WEIGHT_CAP,
					tradable: Tradability::default(),
				}
			);

			assert_pool_state!(13360000000000000, 26720000000000000);
		});
}
