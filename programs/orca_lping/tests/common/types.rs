use std::sync::Arc;

use anchor_lang::prelude::{Pubkey, Rent};
use solana_program_test::{BanksClient, ProgramTestContext};
use solana_sdk::signature::Keypair;

pub struct TestContext {
    pub initial_market_owner: Arc<Keypair>,
    pub context: ProgramTestContext,
    pub rent: Rent,
}

pub struct Env<'a> {
    pub program_id: &'a Pubkey,
    pub client: &'a mut BanksClient,
}

#[derive(Debug, Clone)]
pub struct WhirlpoolAccounts {
    pub whirlpool: Arc<Keypair>,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Arc<Keypair>,
    pub token_b_vault: Arc<Keypair>,
    pub ticks_array: Vec<Pubkey>,
}

#[derive(Debug, Clone)]
pub struct WhirlpoolPositionPrerequisiteAccounts {
    pub position_mint: Arc<Keypair>,
    pub position: Pubkey,
    pub position_bump: u8,
    pub position_metadata: Pubkey,
    pub position_metadata_bump: u8,
    pub position_token_account: Pubkey,
}

#[derive(Debug, Clone)]
pub struct WhirlpoolPosition {
    pub position: Pubkey,
    pub position_token_account: Pubkey,
    pub position_mint: Arc<Keypair>,
    pub lower_index: i32,
    pub higher_index: i32,
}
