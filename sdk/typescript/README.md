# @zeris/sdk

TypeScript SDK for Zeris Zero-Knowledge Resolution & Integrity System.

## Installation

```bash
npm install @zeris/sdk
```

## Usage

```typescript
import { ZerisClient, ProofBuilder } from '@zeris/sdk';
import { Connection, clusterApiUrl } from '@solana/web3.js';

const connection = new Connection(clusterApiUrl('devnet'));
const client = new ZerisClient(connection);

// Build proof from JSON
const proofBuilder = client.getProofBuilder();
const proofData = proofBuilder.buildProofFromJson(jsonProof);

// Submit proof
const signature = await client.submitProof(keypair, proofData);

// Get current state
const state = await client.getZerisState();
console.log('Merkle Root:', state.merkleRoot);
```

## CLI

```bash
# Submit proof
zeris-cli submit-proof -k keypair.json -p proof.json

# Get state
zeris-cli get-state
```

## API Reference

See [API Documentation](https://docs.zeris.io/sdk/typescript/api-reference)