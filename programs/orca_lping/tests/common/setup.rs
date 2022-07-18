use std::sync::Arc;

use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer};

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

pub fn funded_kps<const NUM: usize>(
    test: &mut ProgramTest,
    min_balance_lamports: u64,
) -> [KP; NUM] {
    (0..NUM)
        .map(|_| funded_kp(test, min_balance_lamports))
        .collect::<Vec<KP>>()
        .try_into()
        .unwrap()
}
