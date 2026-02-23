use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

pub fn handler(ctx: Context<CloseVault>) -> Result<()> {
    let vault_state = &ctx.accounts.vault_state;

    require!(
        vault_state.owner == ctx.accounts.owner.key(),
        TimeLockError::Unauthorized
    );

    require!(
        ctx.accounts.vault.to_account_info().lamports() == 0,
        TimeLockError::VaultNotEmpty
    );

    emit!(VaultClosed {
        owner: vault_state.owner,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", owner.key().as_ref()],
        bump = vault_state.state_bump,
        constraint = vault_state.owner == owner.key() @ TimeLockError::Unauthorized,
        close = owner
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
}