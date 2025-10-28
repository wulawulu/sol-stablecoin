use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{SEED_COLLATERAL_ACCOUNT, SEED_SOL_ACCOUNT},
    state::{Collateral, Config},
    SEED_CONFIG_ACCOUNT,
};

#[derive(Accounts)]
pub struct DepositAndMint<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump=config.bump,
        has_one = mint,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub collateral: Account<'info, Collateral>,
    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_account: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process_deposit_and_mint(
    ctx: Context<DepositAndMint>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    msg!("Initialized Config Account: {:#?}", ctx.accounts.config);

    Ok(())
}
