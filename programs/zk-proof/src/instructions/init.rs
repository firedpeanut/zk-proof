use crate::state::*;

use anchor_lang::prelude::*;
use std::ops::DerefMut;

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct InitVerifyingKey<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [
            VERIFYING_KEY_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + Groth16VerifyingKeyState::INIT_SPACE
    )]
    pub verifying_key: Account<'info, Groth16VerifyingKeyState>,
    pub system_program: Program<'info, System>,
}

pub fn process_init(
    ctx: Context<InitVerifyingKey>,
    index: u16,
    vk_alpha_g1: [u8; 64],
    vk_beta_g2: [u8; 128],
    vk_gamma_g2: [u8; 128],
    vk_delta_g2: [u8; 128],
) -> Result<()> {
    let verifying_key = ctx.accounts.verifying_key.deref_mut();

    verifying_key.index = index;
    verifying_key.bump = ctx.bumps.verifying_key;
    verifying_key.vk_alpha_g1 = vk_alpha_g1;
    verifying_key.vk_beta_g2 = vk_beta_g2;
    verifying_key.vk_gamma_g2 = vk_gamma_g2;
    verifying_key.vk_delta_g2 = vk_delta_g2;

    Ok(())
}
