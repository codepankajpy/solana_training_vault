pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5ZcAT1AQBhQdcj7pMo4zq4SvjmqM8NgBZ1nBWRSgAWLp"); // my program ID

// anchor keys sync if any error related to key mismatch

#[program]
pub mod learn_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
