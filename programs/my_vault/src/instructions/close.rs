
use anchor_lang::{prelude::*, system_program};
use crate::{STATE_SEED, VAULT_SEED, VaultState};

#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [STATE_SEED, user.key().as_ref()],
        bump = state.state_bump,
        close = user,
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [VAULT_SEED, state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

pub fn close_vault(ctx: Context<Close>) -> Result<()> {

    let from_pubkey = ctx.accounts.vault.to_account_info();
    let to_pubkey = ctx.accounts.user.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();
    let balance = ctx.accounts.vault.lamports();

    let state_key = ctx.accounts.state.key();
    let vault_bump = ctx.accounts.state.vault_bump;
    let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SEED, state_key.as_ref(), &[vault_bump]]];

    let cpi_account = system_program::Transfer{
        from: from_pubkey,
        to: to_pubkey,
    };

    let cpi_context = CpiContext::new_with_signer(
        program_id.key(),
        cpi_account,
        signer_seeds
    );

    system_program::transfer(
        cpi_context,
        balance
    )?;

    Ok(())
}