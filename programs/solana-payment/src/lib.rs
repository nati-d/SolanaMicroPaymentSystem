use anchor_lang::prelude::*;

declare_id!("9PpJphgktVLGhqGiubf5tpcrKkBQXffUPLr1C65aw7My");

#[program]
pub mod solana_payment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
