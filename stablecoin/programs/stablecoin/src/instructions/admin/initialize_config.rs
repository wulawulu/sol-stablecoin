use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::{
    constants::{LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR},
    state::Config,
    SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = authority,
        mint::freeze_authority = authority,
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_initialize_config(ctx: Context<InitConfig>) -> Result<()> {
    *ctx.accounts.config = Config {
        authority: ctx.accounts.authority.key(),
        mint: ctx.accounts.mint.key(),
        bump: ctx.bumps.config,
        bump_mint: ctx.bumps.mint,
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
    };
    msg!("Initialized Config Account: {:#?}", ctx.accounts.config);

    Ok(())
}
