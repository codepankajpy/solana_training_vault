mod common;

use common::*;
use anchor_lang::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
pub fn test_close(){
    let user = Keypair::new();

    let (vault, _) = vault_pda(&user.pubkey());
    let (state, _) = state_pda(&user.pubkey());

    let mut svm = svm_init();

    get_fund(&mut svm, &user.pubkey(), 1_000_000_000);

    initalize_vault(&user, &mut svm);

    let deposit_amount = 100_000_000;

    deposit_vault(&user, &mut svm, deposit_amount);

    let user_balance_before = svm.get_account(&user.pubkey()).unwrap().lamports;
    
    let ix = build_close_ix(&user.pubkey());

    send_ix(&mut svm, &user, ix);

    let vault_account = svm.get_account(&vault);
    let user_balance_after = svm.get_account(&user.pubkey()).unwrap().lamports;
    let state_account = svm.get_account(&state);

    // assert_eq!(vault_balance_after, 0);
    assert!(vault_account.is_none());
    assert!(user_balance_after > user_balance_before);
    assert!(state_account.is_none());

}