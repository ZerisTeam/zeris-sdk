use zeris_sdk::{ZerisClient, ProofData};
use solana_sdk::signature::{Keypair, Signer};
use std::{fs, str::FromStr};
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Zeris Rust Example");

    // Load or generate keypair
    let keypair = if let Ok(keypair_data) = fs::read_to_string("keypair.json") {
        let secret_key: Vec<u8> = serde_json::from_str(&keypair_data)?;
        Keypair::from_bytes(&secret_key)?
    } else {
        println!("Generating new keypair...");
        let keypair = Keypair::new();
        fs::write("keypair.json", serde_json::to_string(&keypair.to_bytes())?)?;
        println!("Keypair saved to keypair.json");
        keypair
    };

    println!("Public key: {}", keypair.pubkey());

    // Initialize client
    let client = ZerisClient::new("https://api.devnet.solana.com");

    // Mock proof data (in real usage, load from snarkjs output)
    let proof_data = ProofData {
        proof: vec![1, 2, 3, 4, 5], // Mock proof bytes
        public_signals: vec![5, 3, 15], // a * b = c
    };

    println!("Submitting proof...");

    match client.submit_proof(&keypair, &proof_data) {
        Ok(signature) => {
            println!("✅ Proof submitted successfully!");
            println!("Transaction signature: {}", signature);

            // Get updated state
            println!("Fetching current state...");
            match client.get_zeris_state() {
                Ok(state) => {
                    println!("📊 Current state:");
                    println!("   Last Proof Hash: {:?}", state.last_proof_hash);
                    println!("   Merkle Root: {:?}", state.merkle_root);
                    println!("   Proof Count: {}", state.proof_count);
                    println!("   Authority: {}", state.authority);
                }
                Err(e) => eprintln!("Failed to get state: {}", e),
            }
        }
        Err(e) => {
            eprintln!("❌ Error submitting proof: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}