use std::sync::Arc;

use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer};

use super::{fixtures::SOL, types::BorrowingAccounts};

pub type KP = Arc<Keypair>;
pub fn kp() -> KP {
    Arc::new(Keypair::new())
}

pub fn fund_kp(test: &mut ProgramTest, min_balance_lamports: u64, user: Arc<Keypair>) -> KP {
    test.add_account(
        user.pubkey(),
        Account {
            lamports: min_balance_lamports,
            ..Account::default()
        },
    );
    user
}

pub fn funded_kp(test: &mut ProgramTest, min_balance_lamports: u64) -> KP {
    fund_kp(test, min_balance_lamports, kp())
}

impl BorrowingAccounts {
    pub fn new(test: &mut ProgramTest) -> Self {
        BorrowingAccounts {
            initial_market_owner: funded_kp(test, SOL::from(10.0)),
            global_config: kp(),
            oracle_mappings: kp(),
        }
    }
}
