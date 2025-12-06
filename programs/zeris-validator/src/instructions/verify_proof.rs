use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn handler(ctx: Context<VerifyProof>, proof_hash: [u8; 32]) -> Result<()> {
    let zeris_state = &ctx.accounts.zeris_state;

    if zeris_state.last_proof_hash != proof_hash {
        return err!(ZerisError::InvalidProofHash);
    }

    // Emit event
    emit!(ProofVerified {
        proof_hash,
        is_valid: true,
    });

    Ok(())
}

#[event]
pub struct ProofVerified {
    pub proof_hash: [u8; 32],
    pub is_valid: bool,
}