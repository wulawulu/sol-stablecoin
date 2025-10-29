use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::SEED_CONFIG_ACCOUNT,
    error::ErrorCode,
    instructions::{
        calculate_health_factor, get_lamports_from_usd,
        withdraw::utils::{burn_tokens_internal, withdraw_sol_internal},
    },
    state::{Collateral, Config},
};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump=config.bump,
        has_one=mint,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        has_one= sol_account,
    )]
    pub collateral: Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = calculate_health_factor(
        &ctx.accounts.collateral,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;
    require!(
        health_factor < ctx.accounts.config.min_health_factor,
        ErrorCode::AboveMinimumHealthFactor
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;
    let liquidation_bonus = lamports * ctx.accounts.config.liquidation_bonus / 100;
    let amount_to_liquidate = lamports + liquidation_bonus;

    withdraw_sol_internal(
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.collateral.depositor,
        ctx.accounts.collateral.bump_sol_account,
        amount_to_liquidate,
    )?;

    burn_tokens_internal(
        &ctx.accounts.mint,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        &ctx.accounts.token_program,
        amount_to_burn,
    )?;

    let collateral = &mut ctx.accounts.collateral;
    collateral.collateral_amount = ctx.accounts.sol_account.lamports();
    collateral.minted_amount -= amount_to_burn;

    Ok(())
}
