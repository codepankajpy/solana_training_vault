mod common;

use common::*;
use anchor_lang::prelude::*;
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
pub fn test_withdraw(){
    let user = Keypair::new();

    let (vault, _) = vault_pda(&user.pubkey());

    let mut svm = svm_init();

    get_fund(&mut svm, &user.pubkey(), 1_000_000_000);

    initalize_vault(&user, &mut svm);

    let deposit_amount = 100_000_000;

    deposit_vault(&user, &mut svm, deposit_amount);

    let vault_balance_before = svm.get_account(&vault).unwrap().lamports;
    let user_balance_before = svm.get_account(&user.pubkey()).unwrap().lamports;
    
    let withdraw_amount =  100_000;

    let ix = build_withdraw_ix(&user.pubkey(), withdraw_amount);
    send_ix(&mut svm, &user, ix);

    let vault_balance_after = svm.get_account(&vault).unwrap().lamports;
    let user_balance_after = svm.get_account(&user.pubkey()).unwrap().lamports;

    assert_eq!(vault_balance_after, vault_balance_before - withdraw_amount);
    assert!(user_balance_after > user_balance_before);

}