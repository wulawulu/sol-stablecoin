use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    token_2022::{burn, Burn},
    token_interface::{Mint, Token2022, TokenAccount},
};

use crate::constants::SEED_SOL_ACCOUNT;

pub fn withdraw_sol_internal<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    system_program: &Program<'info, System>,
    depositor_key: &Pubkey,
    bump: u8,
    amount: u64,
) -> Result<()> {
    let seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];
    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from,
                to,
            },
            seeds,
        ),
        amount,
    )
}

pub fn burn_tokens_internal<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    authority: &Signer<'info>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}
