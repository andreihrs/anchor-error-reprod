mod common;
use common::{fixtures::SOL, runner::test, setup::funded_kp, types::BorrowingAccounts};
use solana_program_test::tokio;

#[tokio::test]
async fn test() {
    let mut program = test::program();

    let admin = funded_kp(&mut program, SOL::from(10.0));

    let borrowing = BorrowingAccounts::new(&mut program);
    let mut ctx = test::start(program, &admin).await;

    ctx.initialize_market(&borrowing).await.unwrap();
}
