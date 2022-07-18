mod common;
use crate::common::fixtures::ProgramDependency::{METAPLEX, WHIRLPOOL};
use common::{fixtures::setup_empty_market_with_dependencies, runner::token, setup::kp};
use solana_program_test::tokio;

#[tokio::test]
async fn test_whirpool() {
    let mut ctx = setup_empty_market_with_dependencies(&[WHIRLPOOL, METAPLEX]).await;
    let owner = &ctx.initial_market_owner.clone();

    let token_a_mint = kp();
    token::create_mint(&mut ctx, &token_a_mint).await;

    let token_b_mint = kp();
    token::create_mint(&mut ctx, &token_b_mint).await;

    //     let tick_size = 1;
    //     let whirlpool_accounts = setup_whirlpool(
    //         &mut ctx,
    //         &token_a_mint.pubkey(),
    //         &token_b_mint.pubkey(),
    //         tick_size,
    //         1.0,
    //         &owner,
    //     )
    //     .await;
}
