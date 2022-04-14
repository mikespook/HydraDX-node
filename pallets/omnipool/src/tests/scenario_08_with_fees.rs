use super::*;

/// Auto-generated test
#[test]
fn complex_scenario_with_fee_works() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), 0, 100000000000000000),
			(Omnipool::protocol_account(), 2, 2000000000000000),
			(LP1, 100, 5000000000000000),
			(LP1, 200, 5000000000000000),
			(LP2, 100, 1000000000000000),
			(LP3, 100, 2000000000000000),
			(LP3, 200, 300000000000000),
		])
		.with_registered_asset(100)
		.with_registered_asset(200)
		.with_asset_fee((1, 10))
		.with_protocol_fee((2, 10))
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::add_token(
				Origin::root(),
				2,
				1000000000000000,
				FixedU128::from_float(0.5)
			));

			assert_ok!(Omnipool::add_token(
				Origin::root(),
				0,
				10000000000000000,
				FixedU128::from(1)
			));

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
				FixedU128::from_float(1.1)
			));
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP2), 100, 400000000000000));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP3),
				100,
				200,
				110000000000000,
				10000000000000
			));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP2),
				100,
				200,
				50000000000000,
				10000000000000
			));

			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP3), 200, 200000000000000));

			assert_ok!(Omnipool::buy(
				Origin::signed(LP3),
				200,
				100,
				300000000000000,
				100000000000000000
			));

			assert_ok!(Omnipool::remove_liquidity(Origin::signed(LP3), 3, 200000000000000));

			assert_ok!(Omnipool::sell(
				Origin::signed(LP3),
				1,
				200,
				20000000000000,
				10000000000000
			));

			check_balance_approx!(Omnipool::protocol_account(), 0, 100000000000000000u128, 10);
			check_balance_approx!(Omnipool::protocol_account(), 2, 2000000000000000u128, 10);
			check_balance_approx!(Omnipool::protocol_account(), 1, 14202286858172183u128, 10);
			check_balance_approx!(Omnipool::protocol_account(), 100, 4207572842529316u128, 10);
			check_balance_approx!(Omnipool::protocol_account(), 200, 1660505983521800u128, 10);
			check_balance_approx!(LP1, 100, 3000000000000000u128, 10);
			check_balance_approx!(LP1, 200, 3000000000000000u128, 10);
			check_balance_approx!(LP2, 100, 550000000000000u128, 10);
			check_balance_approx!(LP2, 200, 18177604879418u128, 10);
			check_balance_approx!(LP3, 100, 242427157470684u128, 10);
			check_balance_approx!(LP3, 200, 621316411598782u128, 10);
			check_balance_approx!(LP3, 1, 22905929725970u128, 10);

			check_asset_state!(
				2,
				AssetState {
					reserve: 1000000000000000,
					hub_reserve: 500000000000000,
					shares: 1000000000000000,
					protocol_shares: 1000000000000000,
					tvl: 1000000000000000
				}
			);

			check_asset_state!(
				0,
				AssetState {
					reserve: 10000000000000000,
					hub_reserve: 10135598981929739,
					shares: 10000000000000000,
					protocol_shares: 10000000000000000,
					tvl: 10000000000000000
				}
			);

			check_asset_state!(
				100,
				AssetState {
					reserve: 4207572842529316,
					hub_reserve: 1017604072281039,
					shares: 2400000000000000,
					protocol_shares: 2000000000000000,
					tvl: 3120000000000000
				}
			);

			check_asset_state!(
				200,
				AssetState {
					reserve: 1660505983521800,
					hub_reserve: 2727588715617114,
					shares: 2006381428281194,
					protocol_shares: 2000000000000000,
					tvl: 5415177431234228
				}
			);

			check_state!(
				14202286858172183,
				32535177431234228,
				SimpleImbalance {
					value: 37868015292064,
					negative: true
				}
			);
		});
}
