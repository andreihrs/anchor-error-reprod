use super::{runner::test, setup::funded_kp, types::TestContext};

pub enum ProgramDependency {
    WHIRLPOOL,
    METAPLEX,
}

pub async fn setup_empty_market_with_dependencies(
    dependencies: &[ProgramDependency],
) -> TestContext {
    let mut program = test::program(dependencies);

    let admin = funded_kp(&mut program, SOL::from(10.0));

    let mut ctx = test::start(program, &admin).await;

    ctx
}

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
