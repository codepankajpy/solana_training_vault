mod common;

use common::*;
use anchor_lang::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
pub fn test_initialize(){

    let user = Keypair::new();

    let (state_pda, state_bump) = state_pda(&user.pubkey());

    let (vault_pda, vault_bump) = vault_pda(&user.pubkey());

    let mut svm = svm_init();

    get_fund(&mut svm, &user.pubkey(), 1_000_000_000);

    let ix = build_intial_ix(&user.pubkey());

    send_ix(&mut svm, &user, ix);

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