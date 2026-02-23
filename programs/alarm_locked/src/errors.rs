use anchor_lang::prelude::*;

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