use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub vault_bump: u8, // bump seed to derive vault PDA
    pub state_bump: u8, // bump seed to derive vault-state PDA
}

// VaultState is PDA account used to store program state or metatdata

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

// 8 byte is the discriminator, it is used to make the account type unique
// bump only take 1 byte ranging number from 255 to 1
