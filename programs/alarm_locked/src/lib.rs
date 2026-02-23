use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod events;

use instructions::*;

declare_id!("8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo");

#[program]
pub mod alarm_locked {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, unlock_time: i64) -> Result<()> {
        instructions::initialize::handler(ctx, unlock_time)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw::handler(ctx)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        instructions::close_vault::handler(ctx)
    }
}