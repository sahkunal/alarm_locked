use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

pub fn handler(ctx: Context<Initialize>, unlock_time: i64) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    let clock = Clock::get()?;

    require!(
        unlock_time > clock.unix_timestamp,
        TimeLockError::InvalidUnlockTime
    );

    vault_state.owner = ctx.accounts.owner.key();
    vault_state.unlock_time = unlock_time;
    vault_state.vault_bump = ctx.bumps.vault;
    vault_state.state_bump = ctx.bumps.vault_state;
    vault_state.initialized = true;

    emit!(VaultInitialized {
        owner: vault_state.owner,
        unlock_time,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}