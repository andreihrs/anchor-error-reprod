use crate::common::setup::KP;
use crate::common::types::Env;
use anchor_lang::prelude::{Clock, Pubkey};
use solana_program_test::BanksClientError;
use solana_sdk::commitment_config::CommitmentLevel;

use super::types::{BorrowingAccounts, TestContext};

pub mod test {
    use solana_program_test::{processor, ProgramTest};

    use crate::common::types::TestContext;

    use super::*;
    pub fn program() -> ProgramTest {
        let mut program_test = ProgramTest::new(
            "error_reprod",
            error_reprod::ID,
            processor!(error_reprod::entry),
        );

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

impl TestContext {
    pub async fn initialize_market(
        &mut self,
        borrowing_accounts: &BorrowingAccounts,
    ) -> Result<(), BanksClientError> {
        let tx = crate::common::instructions::initialize_market(
            Env {
                program_id: &error_reprod::ID,
                client: &mut self.context.banks_client,
            },
            borrowing_accounts.initial_market_owner.clone(),
            borrowing_accounts,
        )
        .await;

        println!("InitializeMarket Transaction {:?}", tx);
        self.context
            .banks_client
            .process_transaction_with_commitment(tx, CommitmentLevel::Processed)
            .await
    }
}
