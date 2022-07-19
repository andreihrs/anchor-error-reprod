use solana_sdk::instruction::Instruction;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use std::sync::Arc;

use super::types::{BorrowingAccounts, Env};

#[allow(clippy::needless_lifetimes)]
pub async fn initialize_market<'a>(
    env: Env<'a>,
    payer: Arc<Keypair>,
    borrowing_accounts: &BorrowingAccounts,
) -> Transaction {
    let accounts = accounts::initialize_market(borrowing_accounts);
    let data = data::initialize_market();
    let instruction = Instruction {
        program_id: *env.program_id,
        accounts,
        data,
    };

    Transaction::new_signed_with_payer(
        std::slice::from_ref(&instruction),
        Some(&payer.pubkey()),
        &[
            borrowing_accounts.initial_market_owner.as_ref(),
            borrowing_accounts.oracle_mappings.as_ref(),
        ],
        env.client.get_latest_blockhash().await.unwrap(),
    )
}

mod accounts {
    use crate::{readable, writable_signer};
    use anchor_lang::{prelude::AccountMeta, system_program::System, Id};

    use super::*;

    pub fn initialize_market(borrowing: &BorrowingAccounts) -> Vec<AccountMeta> {
        let account_metadatas = vec![
            writable_signer!(borrowing.initial_market_owner.pubkey()),
            writable_signer!(borrowing.oracle_mappings.pubkey()),
            readable!(System::id()),
        ];

        account_metadatas
    }
}

pub(crate) mod data {
    use sha2::{Digest, Sha256};

    pub fn initialize_market() -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&dispatch_sig("global", "initialize_market"));
        data
    }

    pub fn dispatch_sig(namespace: &str, name: &str) -> [u8; 8] {
        let preimage = format!("{}:{}", namespace, name);

        let mut sighash = [0; 8];
        let mut hasher = Sha256::new();
        hasher.update(preimage.as_bytes());
        sighash.copy_from_slice(&hasher.finalize()[..8]);
        sighash
    }
}
