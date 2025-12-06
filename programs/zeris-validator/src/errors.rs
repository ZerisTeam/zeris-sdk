use anchor_lang::prelude::*;

#[error_code]
pub enum ZerisError {
    #[msg("Invalid proof format")]
    InvalidProofFormat,
    #[msg("Proof verification failed")]
    ProofVerificationFailed,
    #[msg("Invalid proof hash")]
    InvalidProofHash,
    #[msg("Merkle root mismatch")]
    MerkleRootMismatch,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid public signals")]
    InvalidPublicSignals,
}