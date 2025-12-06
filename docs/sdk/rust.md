# Rust SDK

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
zeris-sdk = "0.1.0"
```

## Basic Usage

```rust
use zeris_sdk::{ZerisClient, ProofData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ZerisClient::new("https://api.devnet.solana.com");

    // Create proof data
    let proof_data = ProofData {
        proof: vec![/* proof bytes */],
        public_signals: vec![/* signals */],
    };

    // Submit proof
    let signature = client.submit_proof(&keypair, &proof_data)?;
    println!("Transaction signature: {}", signature);

    // Get state
    let state = client.get_zeris_state()?;
    println!("Merkle root: {:?}", state.merkle_root);

    Ok(())
}
```

## Advanced Features

### Batch Operations

```rust
// Submit multiple proofs
let proofs = vec![proof_data1, proof_data2, proof_data3];
for proof in proofs {
    client.submit_proof(&keypair, &proof)?;
}
```

### Error Handling

```rust
match client.submit_proof(&keypair, &proof_data) {
    Ok(signature) => println!("Success: {}", signature),
    Err(e) => match e.downcast_ref::<ZerisError>() {
        Some(ZerisError::InvalidProofFormat) => {
            eprintln!("Invalid proof format");
        }
        Some(ZerisError::ProofVerificationFailed) => {
            eprintln!("Proof verification failed");
        }
        _ => eprintln!("Unknown error: {}", e),
    }
}
```

## Integration with Async Runtimes

### Tokio

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let client = ZerisClient::new(rpc_url);

    // Spawn verification task
    let handle = task::spawn(async move {
        client.verify_proof(&proof_hash).await
    });

    let result = handle.await.unwrap();
}
```

### Async-std

```rust
use async_std::task;

#[async_std::main]
async fn main() {
    let client = ZerisClient::new(rpc_url);
    let result = task::spawn(async move {
        client.validate_merkle_root(&root_hash).await
    }).await;
}
```

## Performance Optimization

### Connection Pooling

```rust
use std::sync::Arc;

// Share client across threads
let client = Arc::new(ZerisClient::new(rpc_url));

// Use in multiple async tasks
let client_clone = Arc::clone(&client);
tokio::spawn(async move {
    client_clone.submit_proof(&keypair, &proof_data).await?;
});
```

### Batch Verification

```rust
// Verify multiple proofs concurrently
let tasks: Vec<_> = proof_hashes.into_iter()
    .map(|hash| {
        let client = client.clone();
        tokio::spawn(async move {
            client.verify_proof(&hash).await
        })
    })
    .collect();

for task in tasks {
    let result = task.await??;
    // Process result
}
```

## Type Definitions

```rust
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
```