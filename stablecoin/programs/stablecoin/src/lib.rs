use anchor_lang::prelude::*;

mod constants;
mod error;
mod instructions;
mod state;

use instructions::*;

use constants::*;
use error::*;
use state::*;

declare_id!("FtbNX5hZ3eypVkWoiRozgJH2bZap9yKcFomC9nFTB2xB");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_and_mint(
        ctx: Context<DepositAndMint>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_and_mint(ctx, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        withdraw_sol_amount: u64,
        burn_tokens_amount: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, withdraw_sol_amount, burn_tokens_amount)
    }
}
