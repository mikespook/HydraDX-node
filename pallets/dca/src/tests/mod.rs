use crate::tests::mock::*;
use crate::{AssetId, Balance, BlockNumber, Order, Recurrence, Schedule, ScheduleId, Trade};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::ConstU32;
use sp_runtime::BoundedVec;

pub mod mock;
pub mod on_initialize;
mod pause;
pub mod resume;
pub mod schedule;
pub mod terminate;

#[macro_export]
macro_rules! assert_balance {
	( $x:expr, $y:expr, $z:expr) => {{
		assert_eq!(Tokens::free_balance($y, &$x), $z);
	}};
}

struct ScheduleBuilder {
	pub period: Option<BlockNumber>,
	pub order: Option<Order<AssetId>>,
	pub recurrence: Option<Recurrence>,
}

impl ScheduleBuilder {
	fn new() -> ScheduleBuilder {
		ScheduleBuilder {
			period: Some(ONE_HUNDRED_BLOCKS),
			recurrence: Some(Recurrence::Fixed(5)),
			order: Some(Order::Buy {
				asset_in: HDX,
				asset_out: BTC,
				amount_out: ONE,
				max_limit: Balance::MAX,
				route: create_bounded_vec(vec![]),
			}),
		}
	}

	fn with_period(mut self, period: BlockNumber) -> ScheduleBuilder {
		self.period = Some(period);
		return self;
	}

	fn with_order(mut self, buy_order: Order<AssetId>) -> ScheduleBuilder {
		self.order = Some(buy_order);
		return self;
	}

	fn with_recurrence(mut self, recurrence: Recurrence) -> ScheduleBuilder {
		self.recurrence = Some(recurrence);
		return self;
	}

	fn build(self) -> Schedule<AssetId> {
		Schedule {
			period: self.period.unwrap(),
			recurrence: self.recurrence.unwrap(),
			order: self.order.unwrap(),
		}
	}
}
pub fn empty_vec() -> BoundedVec<Trade, ConstU32<5>> {
	create_bounded_vec(vec![])
}

pub fn create_bounded_vec(trades: Vec<Trade>) -> BoundedVec<Trade, ConstU32<5>> {
	let bounded_vec: BoundedVec<Trade, sp_runtime::traits::ConstU32<5>> = trades.try_into().unwrap();
	bounded_vec
}

fn assert_scheduled_ids(block: BlockNumberFor<Test>, expected_schedule_ids: Vec<ScheduleId>) {
	//TODO: make this as a macro to better readability and also use it everywhere where we can
	let actual_schedule_ids = DCA::schedule_ids_per_block(block);
	assert!(DCA::schedule_ids_per_block(block).is_some());
	let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(expected_schedule_ids);
	assert_eq!(actual_schedule_ids.unwrap(), expected_scheduled_ids_for_next_block);
}

fn create_bounded_vec_with_schedule_ids(schedule_ids: Vec<ScheduleId>) -> BoundedVec<ScheduleId, ConstU32<5>> {
	let bounded_vec: BoundedVec<ScheduleId, sp_runtime::traits::ConstU32<5>> = schedule_ids.try_into().unwrap();
	bounded_vec
}

//TODO:
/*
## How to handle error in on_initialize:
-using default schedule
(- surely wrap it transactional block - execue_schedules - should be transactional)

- Frido Martin had IncosistentState, might be fuine here too

- Use defensive_ok_or - THIS ONE LOOKS PROMISING


## how extrinsics are executed in the blocks

on_initialize - add ordering - we need to make sure in the runtime config that we put pallets before and after it
*/
