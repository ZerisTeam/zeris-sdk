# ZK Integration Flow

## End-to-End Workflow

```
1. Circuit Design     2. Trusted Setup      3. Proof Generation
     ↓                      ↓                       ↓
   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
   │   Circom     │───▶│   snarkjs    │───▶│   Client    │
   │   Circuit    │    │   Ceremony   │    │   App       │
   └─────────────┘    └─────────────┘    └─────────────┘
                                                          │
4. Proof Submission  5. On-Chain Verify   6. State Update
     ↓                      ↓                       ↓
   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
   │   SDK       │───▶│   Solana     │───▶│   Merkle    │
   │   Client    │    │   Program    │    │   Tree      │
   └─────────────┘    └─────────────┘    └─────────────┘
```

## Detailed Steps

### 1. Circuit Development

```javascript
// circuit.circom
pragma circom 2.0.0;

template Multiplier() {
    signal input a;
    signal input b;
    signal output c;

    c <== a * b;
}

component main = Multiplier();
```

### 2. Compilation & Setup

```bash
# Compile circuit
circom circuit.circom --wasm --r1cs

# Generate trusted setup
snarkjs groth16 setup circuit.r1cs pot12_final.ptau circuit_final.zkey

# Export verification key
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
```

### 3. Client-Side Proof Generation

```typescript
// Generate proof
const { proof, publicSignals } = await snarkjs.groth16.fullProve(
    { a: 3, b: 5 },
    'circuit_js/circuit.wasm',
    'circuit_final.zkey'
);

// Submit to Zeris
const zerisClient = new ZerisClient(connection);
const proofData = proofBuilder.buildProofFromJson({ proof, publicSignals });
await zerisClient.submitProof(keypair, proofData);
```

### 4. On-Chain Verification

The Solana program automatically:

1. Parses the proof data
2. Calls the appropriate verifier (Groth16/Plonk/etc.)
3. Uses Solana syscalls for cryptographic operations
4. Updates the Merkle tree
5. Emits verification events

### 5. Event Monitoring

```typescript
// Listen for verification events
client.listenForEvents((event) => {
    if (event.type === 'ProofVerified') {
        console.log('Proof verified:', event.proofHash);
    }
});
```

## Integration Patterns

### Web Application

```typescript
// React component
const ProofSubmitter = () => {
    const [status, setStatus] = useState('idle');

    const submitProof = async () => {
        setStatus('generating');
        const proof = await generateProof(inputs);
        setStatus('submitting');
        await zerisClient.submitProof(keypair, proof);
        setStatus('verified');
    };

    return <button onClick={submitProof}>Submit Proof</button>;
};
```

### Backend Service

```rust
// Rust service
async fn verify_and_store_proof(proof_data: ProofData) -> Result<()> {
    let client = ZerisClient::new(rpc_url);
    client.submit_proof(&keypair, &proof_data).await?;

    // Store proof hash in database
    database.store_proof_hash(proof_data.calculate_hash()).await?;

    Ok(())
}
```

## Error Handling

```typescript
try {
    await client.submitProof(keypair, proofData);
} catch (error) {
    if (error.code === 'ProofVerificationFailed') {
        console.error('Invalid proof');
    } else if (error.code === 'MerkleRootMismatch') {
        console.error('Integrity check failed');
    }
}
```