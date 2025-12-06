use anchor_lang::prelude::*;
use solana_program::hash::hash;
use solana_program::alt_bn128;

pub trait ProvingSystem {
    fn verify_proof(&self, proof_bytes: &[u8], public_signals: &[u8]) -> Result<bool>;
}

pub struct Groth16Verifier;

impl ProvingSystem for Groth16Verifier {
    fn verify_proof(&self, proof_bytes: &[u8], public_signals: &[u8]) -> Result<bool> {
        // Simplified Groth16 verification using alt_bn128
        // In practice, this would parse the proof and perform pairing checks
        // For now, return true if proof_bytes is not empty
        // TODO: Implement full Groth16 verification
        Ok(!proof_bytes.is_empty())
    }
}

pub fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0; 32];
    }

    let mut current_level = leaves.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in current_level.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() == 2 { chunk[1] } else { left };
            let combined = [left, right].concat();
            let hash = hash(&combined).to_bytes();
            next_level.push(hash);
        }
        current_level = next_level;
    }

    current_level[0]
}

pub fn update_merkle_root(current_root: [u8; 32], new_leaf: [u8; 32]) -> [u8; 32] {
    // Simple update: hash(current_root + new_leaf)
    let combined = [current_root, new_leaf].concat();
    hash(&combined).to_bytes()
}