use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    instructions::{
        check_health_factor,
        deposit::utils::{deposit_sol_internal, mint_tokens_internal},
    },
    Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT,
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
        init_if_needed,
        payer = depositor,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
        space = 0,
        owner = system_program::ID,
    )]
    /// CHECK: System-owned PDA vault for storing deposited SOL; initialized and funded within the instruction.
    pub sol_account: UncheckedAccount<'info>,
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

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process_deposit_and_mint(
    ctx: Context<DepositAndMint>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let sol_account_info = ctx.accounts.sol_account.to_account_info();

    let collateral = &mut ctx.accounts.collateral;
    collateral.collateral_amount = sol_account_info.lamports() + amount_collateral;
    collateral.minted_amount += amount_to_mint;

    if !collateral.is_initialized {
        collateral.is_initialized = true;
        collateral.depositor = ctx.accounts.depositor.key();
        collateral.sol_account = ctx.accounts.sol_account.key();
        collateral.token_account = ctx.accounts.token_account.key();
        collateral.bump = ctx.bumps.collateral;
        collateral.bump_sol_account = ctx.bumps.sol_account;
    }

    check_health_factor(
        &ctx.accounts.collateral,
        &ctx.accounts.config,
        &ctx.accounts.price_update,
    )?;

    deposit_sol_internal(
        &ctx.accounts.depositor,
        sol_account_info.clone(),
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    mint_tokens_internal(
        &ctx.accounts.mint,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        ctx.accounts.config.bump_mint,
        amount_to_mint,
    )?;

    Ok(())
}
