use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofData {
    pub proof: Vec<u8>,
    pub public_signals: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZerisState {
    pub last_proof_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub proof_count: u64,
    pub authority: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitProofEvent {
    pub proof_hash: [u8; 32],
    pub submitter: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofVerifiedEvent {
    pub proof_hash: [u8; 32],
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleRootUpdatedEvent {
    pub new_root: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleRootValidatedEvent {
    pub root_hash: [u8; 32],
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ZerisEvent {
    SubmitProof(SubmitProofEvent),
    ProofVerified(ProofVerifiedEvent),
    MerkleRootUpdated(MerkleRootUpdatedEvent),
    MerkleRootValidated(MerkleRootValidatedEvent),
}