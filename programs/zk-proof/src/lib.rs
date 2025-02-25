pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("GwacL1YSrt8pAzZ5CupAHzzXAcwL65Wk2M4FUxM8fRGo");

#[program]
pub mod zk_proof {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
