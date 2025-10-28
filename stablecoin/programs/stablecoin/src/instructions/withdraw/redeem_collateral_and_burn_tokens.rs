use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    constants::{SEED_COLLATERAL_ACCOUNT, SEED_SOL_ACCOUNT},
    state::Collateral,
};

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [SEED_COLLATERAL_ACCOUNT, signer.key().as_ref()],
        bump,
    )]
    pub collateral: Account<'info, Collateral>,
    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, signer.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process_redeem_collateral_and_burn_tokens() -> Result<()> {
    Ok(())
}
