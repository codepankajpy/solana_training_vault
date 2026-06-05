use anchor_lang::{prelude::*, system_program};

use crate::{VaultState, VAULT_SEED, STATE_SEED};


#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [STATE_SEED, user.key().as_ref()],
        bump = state.state_bump,
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [VAULT_SEED, state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

pub fn deposit_vault(ctx: Context<Deposit>, amount: u64) -> Result<()>{
    

    let cpi_account = system_program::Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        cpi_account
    );

    system_program::transfer(
        cpi_context,
        amount
    )?;

    Ok(())
}