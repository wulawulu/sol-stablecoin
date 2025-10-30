use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT},
    instructions::{
        check_health_factor,
        withdraw::utils::{burn_tokens_internal, withdraw_sol_internal},
    },
    state::{Collateral, Config},
};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump=config.bump,
        has_one=mint,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump=collateral.bump,
        has_one= sol_account,
        has_one=token_account,
    )]
    pub collateral: Account<'info, Collateral>,
    #[account(mut, owner = system_program::ID)]
    /// CHECK: System-owned PDA vault created during deposit; access is restricted through seeds.
    pub sol_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    withdraw_sol_amount: u64,
    burn_tokens_amount: u64,
) -> Result<()> {
    let sol_account_info = ctx.accounts.sol_account.to_account_info();

    let collateral = &mut ctx.accounts.collateral;
    collateral.collateral_amount = sol_account_info.lamports() - withdraw_sol_amount;
    collateral.minted_amount -= burn_tokens_amount;

    check_health_factor(
        &ctx.accounts.collateral,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    burn_tokens_internal(
        &ctx.accounts.mint,
        &ctx.accounts.token_account,
        &ctx.accounts.depositor,
        &ctx.accounts.token_program,
        burn_tokens_amount,
    )?;

    withdraw_sol_internal(
        sol_account_info,
        ctx.accounts.depositor.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.depositor.key(),
        ctx.accounts.collateral.bump_sol_account,
        withdraw_sol_amount,
    )?;

    Ok(())
}
