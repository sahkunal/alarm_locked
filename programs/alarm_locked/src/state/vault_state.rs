use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub unlock_time: i64,
    pub vault_bump: u8,
    pub state_bump: u8,
    pub initialized: bool,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 32 + 8 + 1 + 1 + 1;
}