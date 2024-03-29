use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
	fn bond() -> Weight;
	fn bond_extra() -> Weight;
	fn unbond() -> Weight;
	fn withdraw_unbonded_update(s: u32) -> Weight;
	fn withdraw_unbonded_kill(s: u32) -> Weight;
	fn validate() -> Weight;
	fn kick(k: u32) -> Weight;
	fn nominate(n: u32) -> Weight;
	fn chill() -> Weight;
	fn set_payee() -> Weight;
	fn set_controller() -> Weight;
	fn set_validator_count() -> Weight;
	fn force_no_eras() -> Weight;
	fn force_new_era() -> Weight;
	fn force_new_era_always() -> Weight;
	fn set_invulnerables(v: u32) -> Weight;
	fn force_unstake(s: u32) -> Weight;
	fn cancel_deferred_slash(s: u32) -> Weight;
	fn payout_stakers_dead_controller(n: u32) -> Weight;
	fn payout_stakers_alive_staked(n: u32) -> Weight;
	fn rebond(l: u32) -> Weight;
	fn set_history_depth(e: u32) -> Weight;
	fn reap_stash(s: u32) -> Weight;
	fn new_era(v: u32, n: u32) -> Weight;
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32) -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn bond() -> Weight {
		(76_281_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn bond_extra() -> Weight {
		(62_062_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn unbond() -> Weight {
		(57_195_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_update(s: u32) -> Weight {
		(58_043_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((52_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_kill(s: u32) -> Weight {
		(89_920_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((2_526_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn validate() -> Weight {
		(20_228_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn kick(k: u32) -> Weight {
		(31_066_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((17_754_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	fn nominate(n: u32) -> Weight {
		(33_494_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((5_253_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn chill() -> Weight {
		(19_396_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_payee() -> Weight {
		(13_449_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_controller() -> Weight {
		(29_184_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_validator_count() -> Weight {
		(2_266_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_no_eras() -> Weight {
		(2_462_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era() -> Weight {
		(2_483_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_new_era_always() -> Weight {
		(2_495_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_invulnerables(v: u32) -> Weight {
		(2_712_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn force_unstake(s: u32) -> Weight {
		(60_508_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_525_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn cancel_deferred_slash(s: u32) -> Weight {
		(5_886_772_000 as Weight)
			// Standard Error: 393_000
			.saturating_add((34_849_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn payout_stakers_dead_controller(n: u32) -> Weight {
		(127_627_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((49_354_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn payout_stakers_alive_staked(n: u32) -> Weight {
		(156_838_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((62_653_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn rebond(l: u32) -> Weight {
		(40_110_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((78_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn set_history_depth(e: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((32_883_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	fn reap_stash(s: u32) -> Weight {
		(64_605_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_506_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn new_era(v: u32, n: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 926_000
			.saturating_add((548_212_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 46_000
			.saturating_add((78_343_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 48_000
			.saturating_add((937_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 19_000
			.saturating_add((657_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 48_000
			.saturating_add((70_669_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 101_000
			.saturating_add((7_658_000 as Weight).saturating_mul(w as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(w as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn bond() -> Weight {
		(76_281_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn bond_extra() -> Weight {
		(62_062_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn unbond() -> Weight {
		(57_195_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_update(s: u32) -> Weight {
		(58_043_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((52_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn withdraw_unbonded_kill(s: u32) -> Weight {
		(89_920_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((2_526_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn validate() -> Weight {
		(20_228_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn kick(k: u32) -> Weight {
		(31_066_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((17_754_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	fn nominate(n: u32) -> Weight {
		(33_494_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((5_253_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn chill() -> Weight {
		(19_396_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn set_payee() -> Weight {
		(13_449_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_controller() -> Weight {
		(29_184_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_validator_count() -> Weight {
		(2_266_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_no_eras() -> Weight {
		(2_462_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_new_era() -> Weight {
		(2_483_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_new_era_always() -> Weight {
		(2_495_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_invulnerables(v: u32) -> Weight {
		(2_712_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn force_unstake(s: u32) -> Weight {
		(60_508_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_525_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn cancel_deferred_slash(s: u32) -> Weight {
		(5_886_772_000 as Weight)
			// Standard Error: 393_000
			.saturating_add((34_849_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn payout_stakers_dead_controller(n: u32) -> Weight {
		(127_627_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((49_354_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn payout_stakers_alive_staked(n: u32) -> Weight {
		(156_838_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((62_653_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn rebond(l: u32) -> Weight {
		(40_110_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((78_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn set_history_depth(e: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 70_000
			.saturating_add((32_883_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	fn reap_stash(s: u32) -> Weight {
		(64_605_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_506_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn new_era(v: u32, n: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 926_000
			.saturating_add((548_212_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 46_000
			.saturating_add((78_343_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn submit_solution_better(v: u32, n: u32, a: u32, w: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 48_000
			.saturating_add((937_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 19_000
			.saturating_add((657_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 48_000
			.saturating_add((70_669_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 101_000
			.saturating_add((7_658_000 as Weight).saturating_mul(w as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(a as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(w as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
