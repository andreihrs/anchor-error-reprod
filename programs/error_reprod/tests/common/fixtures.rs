use super::{runner::test, setup::funded_kp, types::TestContext};

use solana_sdk::native_token::sol_to_lamports;

pub struct SOL;
impl SOL {
    pub fn one() -> u64 {
        Self::from(1.0)
    }
    pub fn from(amt: f64) -> u64 {
        sol_to_lamports(amt)
    }
}
