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
pub struct BorrowingAccounts {
    pub initial_market_owner: Arc<Keypair>,
    pub global_config: Arc<Keypair>,
    pub oracle_mappings: Arc<Keypair>,
}
