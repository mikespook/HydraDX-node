use super::*;

/// Auto-generated test
#[test]
fn scenario_06() {
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
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_token(
				Origin::signed(LP1),
				100,
				2000000000000000,
				FixedU128::from_float(0.65)
			));

			assert_ok!(Omnipool::add_token(
				Origin::signed(LP1),
				200,
				2000000000000000,
				FixedU128::from_float(0.65)
			));
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP2), 100, 400000000000000));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP3),
				100,
				200,
				50000000000000,
				10000000000000
			));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP2),
				100,
				200,
				50000000000000,
				10000000000000
			));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP2),
				100,
				200,
				50000000000000,
				10000000000000
			));

			assert_balance_approx!(Omnipool::protocol_account(), 0, NATIVE_AMOUNT, 10);
			assert_balance_approx!(Omnipool::protocol_account(), 2, 1000000000000000u128, 10);
			assert_balance!(Omnipool::protocol_account(), 1, 13360000000000000);
			assert_balance!(Omnipool::protocol_account(), 100, 2550000000000000);
			assert_balance!(Omnipool::protocol_account(), 200, 1868131868131872);
			assert_balance!(LP1, 100, 3000000000000000);
			assert_balance!(LP1, 200, 3000000000000000);
			assert_balance!(LP2, 100, 500000000000000);
			assert_balance!(LP2, 1, 0);
			assert_balance!(LP2, 200, 84059366927890);
			assert_balance!(LP3, 100, 950000000000000);
			assert_balance!(LP3, 1, 0);
			assert_balance!(LP3, 200, 47808764940238);

			assert_asset_state!(
				2,
				AssetReserveState {
					reserve: 1000000000000000,
					hub_reserve: 500000000000000,
					shares: 1000000000000000,
					protocol_shares: 1000000000000000,
					tvl: 1000000000000000,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				0,
				AssetReserveState {
					reserve: 10000000000000000,
					hub_reserve: 10000000000000000,
					shares: 10000000000000000,
					protocol_shares: 10000000000000000,
					tvl: 20000000000000000,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				100,
				AssetReserveState {
					reserve: 2550000000000000,
					hub_reserve: 1468235294117649,
					shares: 2400000000000000,
					protocol_shares: 2000000000000000,
					tvl: 3120000000000000,
					tradable: Tradability::default(),
				}
			);

			assert_asset_state!(
				200,
				AssetReserveState {
					reserve: 1868131868131872,
					hub_reserve: 1391764705882351,
					shares: 2000000000000000,
					protocol_shares: 2000000000000000,
					tvl: 2600000000000000,
					tradable: Tradability::default(),
				}
			);

			assert_pool_state!(13360000000000000, 26720000000000000, SimpleImbalance::default());
		});
}
