use crate::error::ErrorCode;
use crate::state::*;

use anchor_lang::prelude::*;
use solana_program::alt_bn128::prelude::*;
use std::ops::Deref;

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct VerifyingProof<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [
            VERIFYING_KEY_SEED.as_bytes(),
            &index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub verifying_key: Account<'info, Groth16VerifyingKeyState>,
    pub system_program: Program<'info, System>,
}


pub fn proccess_proof(
    ctx: Context<VerifyingProof>,
    index: u16,
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs: [u8; 64],
) -> Result<()> {
    let verifying_key = ctx.accounts.verifying_key.deref();

    let pairing_input = [
            proof_a.as_slice(),
            proof_b.as_slice(),
            public_inputs.as_slice(),
            verifying_key.vk_gamma_g2.as_slice(),
            proof_c.as_slice(),
            verifying_key.vk_delta_g2.as_slice(),
            verifying_key.vk_alpha_g1.as_slice(),
            verifying_key.vk_beta_g2.as_slice(),
        ]
        .concat();

        let pairing_res =alt_bn128_pairing(pairing_input.as_slice()).map_err(|_| ErrorCode::ProofVerificationFailed)?;

    if pairing_res[31] != 1 {
        return err!(ErrorCode::ProofVerificationFailed);
    }

    Ok(())
}