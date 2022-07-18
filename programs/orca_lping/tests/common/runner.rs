use std::sync::Arc;
use std::time::Duration;

use anchor_lang::prelude::{Clock, Pubkey};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::program_error::ProgramError;
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::transport::TransportError;
use solana_sdk::{system_instruction, system_program};
use spl_associated_token_account as ata;
use spl_token::state::Mint;

use crate::common::setup::KP;

use super::types::TestContext;

pub mod test {
    use solana_program_test::{processor, ProgramTest};

    use crate::common::{fixtures::ProgramDependency, types::TestContext};

    use super::*;
    pub fn program(dependencies: &[ProgramDependency]) -> ProgramTest {
        //let serum_program_id = dex::ID;
        //let solend_program_id = Pubkey::new_unique();
        let mut program_test =
            ProgramTest::new("orca_lping", orca_lping::ID, processor!(orca_lping::entry));

        dependencies.iter().for_each(|dep| match dep {
            ProgramDependency::WHIRLPOOL => {
                program_test.add_program("whirlpool", whirlpool::ID, None);
            }
            ProgramDependency::METAPLEX => {
                program_test.add_program("mpl-token-metadata", mpl_token_metadata::ID, None);
            }
        });
        program_test
    }

    pub async fn start(test: ProgramTest, initial_market_owner: &KP) -> TestContext {
        let mut context = test.start_with_context().await;
        let rent = context.banks_client.get_rent().await.unwrap();

        TestContext {
            context,
            rent,
            initial_market_owner: initial_market_owner.clone(),
        }
    }
}

pub mod token {
    use arrayref::array_ref;

    use crate::common::types::TestContext;

    use super::*;

    pub async fn create_ata(env: &mut TestContext, user: &KP, mint: &Pubkey) -> Pubkey {
        let address = ata::get_associated_token_address(&user.pubkey(), mint);
        let instruction =
            ata::create_associated_token_account(&user.pubkey(), &user.pubkey(), mint);
        let transaction = Transaction::new_signed_with_payer(
            std::slice::from_ref(&instruction),
            Some(&user.pubkey()),
            &[user.as_ref()],
            env.context
                .banks_client
                .get_latest_blockhash()
                .await
                .unwrap(),
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();
        address
    }

    pub async fn create_mint(env: &mut TestContext, mint: &KP) {
        let decimals = 6;
        let rent = env.context.banks_client.get_rent().await.unwrap();
        let mint_rent = rent.minimum_balance(Mint::LEN);

        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &env.context.payer.pubkey(),
                    &mint.pubkey(),
                    mint_rent,
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint.pubkey(),
                    &env.initial_market_owner.pubkey(),
                    None,
                    decimals,
                )
                .unwrap(),
            ],
            Some(&env.context.payer.pubkey()),
            &[&env.context.payer, mint.as_ref()],
            env.context.last_blockhash,
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();
    }

    pub async fn mint_to(
        env: &mut TestContext,
        mint: &Pubkey,
        mint_into_account: &Pubkey,
        amount: u64,
    ) -> Result<(), TransportError> {
        let transaction = Transaction::new_signed_with_payer(
            &[spl_token::instruction::mint_to(
                &spl_token::id(),
                mint,
                mint_into_account,
                &env.initial_market_owner.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            Some(&env.initial_market_owner.pubkey()),
            &[&env.initial_market_owner, env.initial_market_owner.as_ref()],
            env.context.last_blockhash,
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await?;
        Ok(())
    }

    pub async fn transfer(
        env: &mut TestContext,
        from: &Pubkey,
        to: &Pubkey,
        signer: &Keypair,
        amount: u64,
    ) -> Result<(), TransportError> {
        let transaction = Transaction::new_signed_with_payer(
            &[spl_token::instruction::transfer(
                &spl_token::id(),
                &from,
                to,
                &signer.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            Some(&signer.pubkey()),
            &[signer],
            env.context.last_blockhash,
        );
        env.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await?;
        Ok(())
    }

    fn check_data_len(data: &[u8], min_len: usize) -> Result<(), ProgramError> {
        if data.len() < min_len {
            Err(ProgramError::AccountDataTooSmall)
        } else {
            Ok(())
        }
    }

    fn get_token_balance(data: &[u8]) -> u64 {
        check_data_len(&data, spl_token::state::Account::get_packed_len()).unwrap();
        let amount = array_ref![data, 64, 8];

        u64::from_le_bytes(*amount)
    }

    pub async fn balance(env: &mut TestContext, account: &Pubkey) -> u64 {
        let acc = env
            .context
            .banks_client
            .get_account(*account)
            .await
            .unwrap()
            .unwrap();

        get_token_balance(&acc.data)
    }
}

impl TestContext {
    pub async fn fast_forward_minutes(&mut self, minutes: u64) {
        self.fast_forward(Duration::from_secs(minutes * 60)).await
    }

    pub async fn fast_forward_seconds(&mut self, seconds: u64) {
        self.fast_forward(Duration::from_secs(seconds)).await
    }

    async fn fast_forward(&mut self, duration: Duration) {
        let mut clock = self
            .context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .unwrap();
        let target = clock.unix_timestamp + duration.as_secs() as i64;

        while clock.unix_timestamp <= target {
            // The exact time is not deterministic, we have to keep wrapping by arbitrary 400 slots
            self.context.warp_to_slot(clock.slot + 2 * 400).unwrap();
            clock = self
                .context
                .banks_client
                .get_sysvar::<Clock>()
                .await
                .unwrap();
        }
    }

    pub async fn get_now_timestamp(&mut self) -> u64 {
        let clock: Clock = self
            .context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .unwrap();
        clock.unix_timestamp as u64
    }

    pub async fn new_keypair(&mut self, min_lamports: u64) -> Arc<Keypair> {
        let account = Keypair::new();
        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::create_account(
                &self.context.payer.pubkey(),
                &account.pubkey(),
                min_lamports,
                0,
                &system_program::id(),
            )],
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer, &account],
            self.context.last_blockhash,
        );

        self.context
            .banks_client
            .process_transaction_with_commitment(transaction, CommitmentLevel::Processed)
            .await
            .unwrap();

        Arc::new(account)
    }

    pub async fn mint_to(
        &mut self,
        mint: &Pubkey,
        mint_into_account: &Pubkey,
        amount: u64,
    ) -> Result<(), TransportError> {
        token::mint_to(self, mint, mint_into_account, amount).await
    }
}
