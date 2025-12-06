# Zeris Rust Example

This example demonstrates how to use the Zeris Rust SDK for high-performance proof submission and verification.

## Prerequisites

- Rust 1.70+
- A Solana keypair with some DEV tokens

## Setup

```bash
# Build the example
cargo build --release
```

## Generate Keypair (if needed)

The example will generate a new keypair if `keypair.json` doesn't exist. For testing, you'll need some DEV tokens:

```bash
# Get DEV tokens (replace with your public key)
solana airdrop 2 <your-public-key>
```

## Run the Example

```bash
cargo run
```

This will:
1. Load or generate a keypair
2. Create mock proof data
3. Submit the proof using the Rust SDK
4. Display the transaction signature
5. Fetch and display the updated program state

## Expected Output

```
Zeris Rust Example
Public key: <your-public-key>
Submitting proof...
✅ Proof submitted successfully!
Transaction signature: <transaction-signature>
Fetching current state...
📊 Current state:
   Last Proof Hash: [1, 2, 3, ...]
   Merkle Root: [4, 5, 6, ...]
   Proof Count: 1
   Authority: <authority-pubkey>
```

## Advanced Usage

### Batch Verification

```rust
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(ZerisClient::new("https://api.devnet.solana.com"));
    let proof_hashes = vec![/* array of proof hashes */];

    let tasks: Vec<_> = proof_hashes.into_iter()
        .map(|hash| {
            let client = Arc::clone(&client);
            tokio::spawn(async move {
                client.verify_proof(&hash)
            })
        })
        .collect();

    for task in tasks {
        task.await??;
    }

    println!("All proofs verified successfully!");
    Ok(())
}
```

### Integration with Backend Services

```rust
use zeris_sdk::models::ZerisState;

async fn monitor_zeris_state(client: &ZerisClient) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let state = client.get_zeris_state()?;
        println!("Current proof count: {}", state.proof_count);

        // Store in database or trigger actions
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
```