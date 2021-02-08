use crate::Balances;
use crate::Balance;
use sp_runtime::traits::Convert;

/// Struct that handles the conversion of Balance -> `u64`. This is used for staking's election
/// calculation.
pub struct U128CurrencyToVote;

impl U128CurrencyToVote {
	fn factor() -> Balance {
		(Balances::total_issuance() / u64::max_value() as Balance).max(1)
	}
}

impl Convert<Balance, u64> for U128CurrencyToVote {
	fn convert(x: Balance) -> u64 {
		(x / Self::factor()) as u64
	}
}

impl Convert<u128, Balance> for U128CurrencyToVote {
	fn convert(x: u128) -> Balance {
		x * Self::factor()
	}
}