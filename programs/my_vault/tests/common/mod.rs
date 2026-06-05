
use anchor_lang::{prelude::*, InstructionData};

use litesvm::{LiteSVM, types::{FailedTransactionMetadata, TransactionMetadata}};
use solana_keypair::Keypair;
use solana_message::{Instruction, Message};
use solana_signer::Signer;
use solana_transaction::{Transaction, TransactionError};


pub const STATE_SEED: &[u8] = b"state_seed";

pub const VAULT_SEED: &[u8] = b"vault_seed";


pub fn svm_init() -> LiteSVM {
    let mut svm = LiteSVM::new();
    let program_id = my_vault::id();
    
    // blockchain must know which program we are calling 
    let program_bytes = include_bytes!("../../../../target/deploy/my_vault.so");
    svm.add_program(program_id, program_bytes).unwrap();
    svm
}

pub fn get_fund(svm: &mut LiteSVM, pubkey: &Pubkey, lamports: u64) {
    svm.airdrop(pubkey, lamports).unwrap();
}

pub fn state_pda(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[STATE_SEED, &user.key().as_ref()],
        &my_vault::id()
    )
}

pub fn vault_pda(user: &Pubkey) -> (Pubkey, u8) {
    let (state, _) = state_pda(user);

    Pubkey::find_program_address(
        &[VAULT_SEED, state.key().as_ref()],
        &my_vault::id()
    )
}

pub fn build_intial_ix(user: &Pubkey) -> Instruction {
    let (state, _) = state_pda(user);
    let (vault, _) = vault_pda(user);

    Instruction::new_with_bytes(
        my_vault::id(),
        &my_vault::instruction::Initialize{}.data(),
        my_vault::accounts::Initialize{
            user: *user,
            state,
            vault,
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

pub fn build_deposit_ix(user: &Pubkey, amount: u64) -> Instruction {
    let (state, _) = state_pda(user);
    let (vault, _) = vault_pda(user);

    Instruction::new_with_bytes(
        my_vault::id(),
        &my_vault::instruction::Deposit{amount}.data(),
        my_vault::accounts::Deposit{
            user: *user,
            state,
            vault,
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

pub fn build_withdraw_ix(user: &Pubkey, amount: u64) -> Instruction {
    let (state, _) = state_pda(user);
    let (vault, _) = vault_pda(user);

    Instruction::new_with_bytes(
        my_vault::id(),
        &my_vault::instruction::Withdraw{amount}.data(),
        my_vault::accounts::Withdraw{
            user: *user, 
            state,
            vault,
            system_program: system_program::ID,
        }.to_account_metas(None),
    )
}

pub fn send_ix(svm: &mut LiteSVM, user: &Keypair, instruction: Instruction) {
    let blockhash = svm.latest_blockhash();
    
    let message = Message::new_with_blockhash(
        &[instruction],
        Some(&user.pubkey()),
        &blockhash    
    );

    let tx = Transaction::new(
        &[user],
        message,
        blockhash
    );

    svm.send_transaction(tx).unwrap();
}

pub fn initalize_vault(user: &Keypair, svm: &mut LiteSVM) {
    let ix = build_intial_ix(&user.pubkey());
    send_ix(svm, user, ix);
}

pub fn deposit_vault(user: &Keypair, svm: &mut LiteSVM, amount: u64) {
    let ix = build_deposit_ix(&user.pubkey(), amount);
    send_ix(svm, user, ix);
}