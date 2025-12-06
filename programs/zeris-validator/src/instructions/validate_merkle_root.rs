use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

pub fn handler(ctx: Context<ValidateMerkleRoot>, root_hash: [u8; 32]) -> Result<()> {
    let zeris_state = &ctx.accounts.zeris_state;

    if zeris_state.merkle_root != root_hash {
        return err!(ZerisError::MerkleRootMismatch);
    }

    // Emit event
    emit!(MerkleRootValidated {
        root_hash,
        is_valid: true,
    });

    Ok(())
}

#[event]
pub struct MerkleRootValidated {
    pub root_hash: [u8; 32],
    pub is_valid: bool,
}