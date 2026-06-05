use anchor_lang::{InstructionData, prelude::*};
use litesvm::LiteSVM;
use my_vault::{STATE_SEED, VAULT_SEED};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_transaction::{Instruction, Transaction};
use solana_signer::Signer;

#[test]
pub fn test_initialize(){

    let user = Keypair::new();
    let program_id = my_vault::id();

    let (state_pda, state_bump) = Pubkey::find_program_address(
        &[STATE_SEED, user.pubkey().as_ref()],
        &program_id
    );

    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[VAULT_SEED, state_pda.as_ref()],
        &program_id
    );

    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/my_vault.so");
    
    svm.add_program(program_id, program_bytes).unwrap();

    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();

    // took from the /target/deploy/idl/
    let data = my_vault::instruction::Initialize{}.data();

    let accounts = my_vault::accounts::Initialize{
        user: user.pubkey(),
        state: state_pda,
        vault: vault_pda,
        system_program: system_program::ID,
    }.to_account_metas(None);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        accounts
    );

    let blockhash = svm.latest_blockhash();

    let message =  Message::new_with_blockhash(
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

    let state_account = svm.get_account(&state_pda).expect("State account should exist.");
    let vault_account = svm.get_account(&vault_pda).expect("Vault account should exist.");

    let account_data = &state_account.data;
    assert_eq!(state_account.owner, my_vault::id());
    assert_eq!(vault_account.owner, system_program::ID);

    assert_eq!(account_data[8], state_bump);
    assert_eq!(account_data[9], vault_bump);

    let vault_balance = svm.get_balance(&vault_pda).expect("Balance should be something.");
    assert!(vault_balance > 0, "vault contain rent-exempt lamports");

}