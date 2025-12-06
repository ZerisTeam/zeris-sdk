use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use crate::models::*;

#[derive(Clone)]
pub struct ZerisClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl ZerisClient {
    pub fn new(rpc_url: &str) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        let program_id = Pubkey::from_str("ZER1S11111111111111111111111111111111111111").unwrap();

        Self {
            rpc_client,
            program_id,
        }
    }

    pub fn submit_proof(
        &self,
        signer: &Keypair,
        proof_data: &ProofData,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (state_pda, _) = Pubkey::find_program_address(&[b"zeris-state"], &self.program_id);

        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &[
                &[0], // submit_proof instruction
                &proof_data.proof,
                &proof_data.public_signals,
            ]
            .concat(),
            vec![
                AccountMeta::new(state_pda, false),
                AccountMeta::new(signer.pubkey(), true),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[signer],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    pub fn verify_proof(
        &self,
        proof_hash: &[u8; 32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (state_pda, _) = Pubkey::find_program_address(&[b"zeris-state"], &self.program_id);

        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &[&[1], proof_hash].concat(), // verify_proof instruction
            vec![AccountMeta::new(state_pda, false)],
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_unsigned(recent_blockhash);
        let transaction = transaction.add(instruction);

        // Since it's verification, we might not need to sign, but for simplicity
        // In practice, you might want to add a signer for logging
        self.rpc_client.simulate_transaction(&transaction)?;
        Ok(())
    }

    pub fn validate_merkle_root(
        &self,
        root_hash: &[u8; 32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (state_pda, _) = Pubkey::find_program_address(&[b"zeris-state"], &self.program_id);

        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &[&[2], root_hash].concat(), // validate_merkle_root instruction
            vec![AccountMeta::new_readonly(state_pda, false)],
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_unsigned(recent_blockhash);
        let transaction = transaction.add(instruction);

        self.rpc_client.simulate_transaction(&transaction)?;
        Ok(())
    }

    pub fn get_zeris_state(&self) -> Result<ZerisState, Box<dyn std::error::Error>> {
        let (state_pda, _) = Pubkey::find_program_address(&[b"zeris-state"], &self.program_id);

        let account = self.rpc_client.get_account(&state_pda)?;
        let data = account.data;

        // Parse account data
        let last_proof_hash: [u8; 32] = data[8..40].try_into().unwrap();
        let merkle_root: [u8; 32] = data[40..72].try_into().unwrap();
        let proof_count = u64::from_le_bytes(data[72..80].try_into().unwrap());
        let authority = Pubkey::new_from_array(data[80..112].try_into().unwrap());

        Ok(ZerisState {
            last_proof_hash,
            merkle_root,
            proof_count,
            authority,
        })
    }
}