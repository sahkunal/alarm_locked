use anchor_lang::prelude::*;

declare_id!("8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo");

#[program]
pub mod alarm_locked {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, unlock_time: i64) -> Result<()> {
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
            unlock_time: vault_state.unlock_time,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault_state = &ctx.accounts.vault_state;
        let clock = Clock::get()?;

        require!(vault_state.initialized, TimeLockError::VaultNotInitialized);
        require!(
            clock.unix_timestamp < vault_state.unlock_time,
            TimeLockError::VaultExpired
        );

        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                transfer_instruction,
            ),
            amount,
        )?;

        emit!(Deposited {
            owner: vault_state.owner,
            amount,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
   pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let vault_state = &ctx.accounts.vault_state;
    let clock = Clock::get()?;

    // Verify ownership
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
        &[vault_state.vault_bump]
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
            signer, // 👈 This proves the program owns the vault
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

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
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
}

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

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        bump = vault_state.vault_bump, // 👈 Use stored bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
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

#[error_code]
pub enum TimeLockError {
    #[msg("Unauthorized access to vault")]
    Unauthorized,
    #[msg("Vault not initialized")]
    VaultNotInitialized,
    #[msg("Vault is still locked")]
    VaultStillLocked,
    #[msg("Vault has expired")]
    VaultExpired,
    #[msg("Invalid unlock time")]
    InvalidUnlockTime,
    #[msg("Vault is not empty")]
    VaultNotEmpty,
    #[msg("Arithmetic overflow")]
    Overflow,
}