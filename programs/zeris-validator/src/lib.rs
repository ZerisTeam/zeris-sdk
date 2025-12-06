use anchor_lang::prelude::*;
use crate::instructions::*;
use crate::state::*;
use crate::errors::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod utils;

declare_id!("ZER1S11111111111111111111111111111111111111");

#[program]
pub mod zeris_validator {
    use super::*;

    pub fn submit_proof(
        ctx: Context<SubmitProof>,
        proof_bytes: Vec<u8>,
        public_signals: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_proof::handler(ctx, proof_bytes, public_signals)
    }

    pub fn verify_proof(ctx: Context<VerifyProof>, proof_hash: [u8; 32]) -> Result<()> {
        instructions::verify_proof::handler(ctx, proof_hash)
    }

    pub fn validate_merkle_root(ctx: Context<ValidateMerkleRoot>, root_hash: [u8; 32]) -> Result<()> {
        instructions::validate_merkle_root::handler(ctx, root_hash)
    }
}