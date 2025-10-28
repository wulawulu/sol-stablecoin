use anchor_lang::prelude::*;

declare_id!("FtbNX5hZ3eypVkWoiRozgJH2bZap9yKcFomC9nFTB2xB");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
