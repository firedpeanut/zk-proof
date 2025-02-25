use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const VERIFYING_KEY_SEED: &str = "verifying_key_seed";
#[account]
#[derive(Debug, InitSpace)]
pub struct Groth16VerifyingKeyState {
    pub bump: u8,
    pub index: u16,       
    pub vk_alpha_g1: [u8; 64],
    pub vk_beta_g2: [u8; 128],
    pub vk_gamma_g2: [u8; 128],
    pub vk_delta_g2: [u8; 128],
    pub padding: [u8; 8],
}