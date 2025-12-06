use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::utils::*;

pub fn handler(
    ctx: Context<SubmitProof>,
    proof_bytes: Vec<u8>,
    public_signals: Vec<u8>,
) -> Result<()> {
    let zeris_state = &mut ctx.accounts.zeris_state;

    // Verify the proof using Groth16 verifier
    let verifier = Groth16Verifier {};
    let is_valid = verifier.verify_proof(&proof_bytes, &public_signals)
        .map_err(|_| error!(ZerisError::ProofVerificationFailed))?;

    if !is_valid {
        return err!(ZerisError::ProofVerificationFailed);
    }

    // Compute proof hash
    let proof_hash = hash(&proof_bytes).to_bytes();

    // Update state
    zeris_state.last_proof_hash = proof_hash;
    zeris_state.merkle_root = update_merkle_root(zeris_state.merkle_root, proof_hash);
    zeris_state.proof_count += 1;
    zeris_state.authority = ctx.accounts.signer.key();
    zeris_state.bump = ctx.bumps.zeris_state;

    // Emit event
    emit!(ProofSubmitted {
        proof_hash,
        submitter: ctx.accounts.signer.key(),
    });

    emit!(ProofVerified {
        proof_hash,
        is_valid: true,
    });

    emit!(MerkleRootUpdated {
        new_root: zeris_state.merkle_root,
    });

    Ok(())
}

#[event]
pub struct ProofSubmitted {
    pub proof_hash: [u8; 32],
    pub submitter: Pubkey,
}

#[event]
pub struct ProofVerified {
    pub proof_hash: [u8; 32],
    pub is_valid: bool,
}

#[event]
pub struct MerkleRootUpdated {
    pub new_root: [u8; 32],
}