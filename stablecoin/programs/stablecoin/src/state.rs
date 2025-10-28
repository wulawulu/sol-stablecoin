use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
    pub bump_mint: u8,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub collateral_amount: u64,
    pub minted_amount: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
    pub is_initialized: bool,
}
