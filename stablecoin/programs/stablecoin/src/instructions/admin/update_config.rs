use anchor_lang::prelude::*;

use crate::{state::Config, SEED_CONFIG_ACCOUNT};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump=config.bump,
    )]
    pub config: Account<'info, Config>,
}

pub fn process_update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.min_health_factor = min_health_factor;

    msg!("Update Config Account: {:#?}", config);
    Ok(())
}
