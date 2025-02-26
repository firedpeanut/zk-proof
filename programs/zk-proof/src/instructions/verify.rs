use crate::error::ErrorCode;

use anchor_lang::prelude::*;
use solana_bn254::prelude::*;

#[derive(Accounts)]
pub struct Verifying<'info> {
    #[account(mut)]
    pub singer: Signer<'info>,
}

pub fn process_verify(
    _ctx: Context<Verifying>,
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_inputs: [u8; 64],
    vk_alpha_g1: [u8; 64],
    vk_beta_g2: [u8; 128],
    vk_gamma_g2: [u8; 128],
    vk_delta_g2: [u8; 128],
) -> Result<()> {
    let pairing_input = [
        proof_a.as_slice(),
        proof_b.as_slice(),
        public_inputs.as_slice(),
        vk_gamma_g2.as_slice(),
        proof_c.as_slice(),
        vk_delta_g2.as_slice(),
        vk_alpha_g1.as_slice(),
        vk_beta_g2.as_slice(),
    ]
    .concat();

    let pairing_res = alt_bn128_pairing(pairing_input.as_slice())
        .map_err(|_| ErrorCode::ProofVerificationFailed)?;

    if pairing_res[31] != 1 {
        return err!(ErrorCode::ProofVerificationFailed);
    }

    Ok(())
}
