pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GwacL1YSrt8pAzZ5CupAHzzXAcwL65Wk2M4FUxM8fRGo");

#[program]
pub mod zk_proof {
    use super::*;

    pub fn init(
        ctx: Context<InitVerifyingKey>,
        index: u16,
        vk_alpha_g1: [u8; 64],
        vk_beta_g2: [u8; 128],
        vk_gamma_g2: [u8; 128],
        vk_delta_g2: [u8; 128],
    ) -> Result<()> {
        process_init(
            ctx,
            index,
            vk_alpha_g1,
            vk_beta_g2,
            vk_gamma_g2,
            vk_delta_g2,
        )
    }

    pub fn proof(
        ctx: Context<VerifyingProof>,
        index: u16,
        proof_a: [u8; 64],
        proof_b: [u8; 128],
        proof_c: [u8; 64],
        public_inputs: [u8; 64],
    ) -> Result<()> {
        process_proof(ctx, index, proof_a, proof_b, proof_c, public_inputs)
    }

    pub fn verify(
        ctx: Context<Verifying>,
        proof_a: [u8; 64],
        proof_b: [u8; 128],
        proof_c: [u8; 64],
        public_inputs: [u8; 64],
        vk_alpha_g1: [u8; 64],
        vk_beta_g2: [u8; 128],
        vk_gamma_g2: [u8; 128],
        vk_delta_g2: [u8; 128],
    ) -> Result<()> {
        process_verify(
            ctx,
            proof_a,
            proof_b,
            proof_c,
            public_inputs,
            vk_alpha_g1,
            vk_beta_g2,
            vk_gamma_g2,
            vk_delta_g2,
        )
    }
}
