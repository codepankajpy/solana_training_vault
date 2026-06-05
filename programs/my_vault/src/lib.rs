pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4KW9NMLsWje1pQVvc8f11tupdUDKeNJh7N15rG9eDqQv"); // my program ID

// anchor keys sync if any error related to key mismatch

#[program]
pub mod learn_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize_vault(ctx)
    }
}
