use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod error_reprod {
    use super::*;

    pub fn initialize_market(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin_authority: Signer<'info>,
    #[account(init, payer = admin_authority,  space = 8 + std::mem::size_of::<OracleMappings>())]
    pub oracle_mappings: Box<Account<'info, OracleMappings>>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug)]
pub struct OracleMappings {
    pub _placeholder0: Pubkey,
    // Validated pyth accounts
    pub pyth_1_price_info: Pubkey,
    pub pyth_2_price_info: Pubkey,
    // pub pyth_3_price_info: Pubkey,
    // pub pyth_4_price_info: Pubkey,
    // pub pyth_5_price_info: Pubkey,
    // pub pyth_6_price_info: Pubkey,
    // pub pyth_7_price_info: Pubkey,
    pub price_pk: Pubkey,
    // All reserved is now 124 u64
    pub _reserved: [u64; 64],
    pub _reserved2: [u64; 32],
    pub _reserved3: [u64; 28],
}
