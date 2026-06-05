use anchor_lang::{prelude::*, system_program};

use crate::{VaultState, VAULT_SEED, STATE_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // the user who inititate this will sign the transaction
    #[account(
        init, // it defines that we are creating a new account
        payer = user, // user will bear the account creation cost
        seeds = [STATE_SEED, user.key().as_ref()], // user key used because so creating the state account is deterministic
        bump,
        space = VaultState::INIT_SPACE,  // discriminator + struct size
    )]
    pub state: Account<'info, VaultState>, // PDA account containing vault state data 
    #[account(
        mut, // why use this????
        seeds = [VAULT_SEED, state.key().as_ref()], // state key used here because the state account will store all the lamports in this account so needed a deterministic way
        bump,
    )]
    pub vault: SystemAccount<'info>, // it will store lamports
    pub system_program: Program<'info, System> // since new account will be created by system program, and all other operation related to blockchain managed by this
}

// impl<'info> Initialize<'info> {
//     pub fn initialize(&mut self, bumps: &InitializeBumps) -> ProgramResult {
//         self.state.state_bump = bumps.state; 
//         self.state.vault_bump = bumps.vault;

//         Ok(())

//         // storing the bumps in the vault and state account so no need to call again and again and to save compute
//     }
// }


// explain this code and understand what is going on here

pub fn initialize_vault(ctx: Context<Initialize>) -> Result<()> {
    let cpi_account = system_program::Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.key(),
        cpi_account
    );

    let rent = Rent::get()?;
    system_program::transfer(
        cpi_context,
        rent.minimum_balance(ctx.accounts.vault.data_len())    
    )?;

    ctx.accounts.state.set_inner(VaultState { 
        vault_bump: ctx.bumps.vault,
        state_bump: ctx.bumps.state, 
    });

    Ok(())
}