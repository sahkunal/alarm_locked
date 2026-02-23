use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    let vault_state = &ctx.accounts.vault_state;
    let clock = Clock::get()?;

    require!(
        vault_state.owner == ctx.accounts.owner.key(),
        TimeLockError::Unauthorized
    );

    require!(vault_state.initialized, TimeLockError::VaultNotInitialized);

    require!(
        clock.unix_timestamp >= vault_state.unlock_time,
        TimeLockError::VaultStillLocked
    );

    let vault_balance = ctx.accounts.vault.to_account_info().lamports();

    let vault_state_key = vault_state.key();
    let seeds = &[
        b"vault",
        vault_state_key.as_ref(),
        &[vault_state.vault_bump],
    ];
    let signer = &[&seeds[..]];

    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
            signer,
        ),
        vault_balance,
    )?;

    emit!(Withdrawn {
        owner: vault_state.owner,
        amount: vault_balance,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", owner.key().as_ref()],
        bump = vault_state.state_bump,
        constraint = vault_state.owner == owner.key() @ TimeLockError::Unauthorized,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}