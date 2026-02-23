use anchor_lang::prelude::*;

#[event]
pub struct VaultInitialized {
    pub owner: Pubkey,
    pub unlock_time: i64,
    pub timestamp: i64,
}

#[event]
pub struct Deposited {
    pub owner: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct Withdrawn {
    pub owner: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct VaultClosed {
    pub owner: Pubkey,
    pub timestamp: i64,
}