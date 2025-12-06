# TypeScript SDK

## Installation

```bash
npm install @zeris/sdk
```

## Basic Usage

```typescript
import { ZerisClient } from '@zeris/sdk';
import { Connection, clusterApiUrl } from '@solana/web3.js';

const connection = new Connection(clusterApiUrl('devnet'));
const client = new ZerisClient(connection);

// Submit a proof
const signature = await client.submitProof(keypair, proofData);

// Get current state
const state = await client.getZerisState();
console.log('Merkle Root:', state.merkleRoot);
```

## Advanced Features

### Proof Builder

```typescript
const proofBuilder = client.getProofBuilder();

// Build from JSON
const proofData = proofBuilder.buildProofFromJson({
    proof: groth16Proof,
    publicSignals: [/* ... */]
});

// Compress for transmission
const compressed = proofBuilder.compressProof(proofData);
const base58Encoded = proofBuilder.toBase58(compressed);
```

### Merkle Syncer

```typescript
const syncer = client.getMerkleSyncer();

// Sync latest root
const root = await syncer.syncMerkleRoot();

// Validate local root
const isValid = await syncer.validateLocalMerkleRoot(localRoot);
```

### Event Listening

```typescript
await client.listenForEvents((event) => {
    switch (event.type) {
        case 'ProofSubmitted':
            console.log('New proof submitted:', event.proofHash);
            break;
        case 'ProofVerified':
            console.log('Proof verified:', event.isValid);
            break;
        case 'MerkleRootUpdated':
            console.log('Merkle root updated:', event.newRoot);
            break;
    }
});
```

## CLI Usage

```bash
# Install globally
npm install -g @zeris/sdk

# Submit proof
zeris-cli submit-proof -k keypair.json -p proof.json

# Verify proof
zeris-cli verify-proof -h <proof-hash>

# Get state
zeris-cli get-state
```

## Error Handling

```typescript
try {
    await client.submitProof(keypair, proofData);
} catch (error) {
    switch (error.code) {
        case 'InvalidProofFormat':
            // Handle invalid proof
            break;
        case 'ProofVerificationFailed':
            // Handle verification failure
            break;
        case 'MerkleRootMismatch':
            // Handle integrity issues
            break;
    }
}
```

## Type Definitions

```typescript
interface ProofData {
    proof: Uint8Array;
    publicSignals: Uint8Array;
}

interface ZerisState {
    lastProofHash: Uint8Array;
    merkleRoot: Uint8Array;
    proofCount: number;
    authority: PublicKey;
}
```