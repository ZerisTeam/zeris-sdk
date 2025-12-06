# API Reference

## TypeScript SDK

### ZerisClient

#### Constructor

```typescript
new ZerisClient(connection: Connection, programId?: PublicKey)
```

#### Methods

##### submitProof

```typescript
submitProof(signer: Keypair, proofData: ProofData): Promise<string>
```

Submits a zero-knowledge proof for verification.

**Parameters:**
- `signer`: Keypair for transaction signing
- `proofData`: Proof data containing proof and public signals

**Returns:** Transaction signature string

**Throws:** `InvalidProofFormat`, `ProofVerificationFailed`

##### verifyProof

```typescript
verifyProof(proofHash: Uint8Array): Promise<void>
```

Verifies a proof hash against stored state.

**Parameters:**
- `proofHash`: 32-byte proof hash

**Throws:** `InvalidProofHash`

##### validateMerkleRoot

```typescript
validateMerkleRoot(rootHash: Uint8Array): Promise<void>
```

Validates a Merkle root against stored state.

**Parameters:**
- `rootHash`: 32-byte Merkle root hash

**Throws:** `MerkleRootMismatch`

##### getZerisState

```typescript
getZerisState(): Promise<ZerisState>
```

Retrieves current Zeris program state.

**Returns:** Current state object

##### listenForEvents

```typescript
listenForEvents(callback: (event: ZerisEvent) => void): Promise<void>
```

Sets up event listeners for program events.

**Parameters:**
- `callback`: Function called for each event

### ProofBuilder

#### buildProofFromJson

```typescript
buildProofFromJson(jsonProof: any): ProofData
```

Builds proof data from JSON format.

**Parameters:**
- `jsonProof`: JSON object with proof and publicSignals

**Returns:** Structured proof data

#### compressProof

```typescript
compressProof(proofData: ProofData): Uint8Array
```

Compresses proof data for transmission.

**Returns:** Compressed byte array

#### toBase58

```typescript
toBase58(data: Uint8Array): string
```

Converts byte array to base58 string.

**Returns:** Base58 encoded string

### MerkleSyncer

#### syncMerkleRoot

```typescript
syncMerkleRoot(): Promise<Uint8Array>
```

Fetches latest Merkle root from chain.

**Returns:** 32-byte Merkle root

#### validateLocalMerkleRoot

```typescript
validateLocalMerkleRoot(localRoot: Uint8Array): Promise<boolean>
```

Validates local Merkle root against on-chain state.

**Returns:** Validation result

## Rust SDK

### ZerisClient

#### new

```rust
pub fn new(rpc_url: &str) -> Self
```

Creates a new Zeris client.

**Parameters:**
- `rpc_url`: Solana RPC endpoint URL

#### submit_proof

```rust
pub fn submit_proof(
    &self,
    signer: &Keypair,
    proof_data: &ProofData
) -> Result<String, Box<dyn std::error::Error>>
```

Submits a proof for verification.

**Returns:** Transaction signature

#### verify_proof

```rust
pub fn verify_proof(&self, proof_hash: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>>
```

Verifies a proof hash.

#### validate_merkle_root

```rust
pub fn validate_merkle_root(&self, root_hash: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>>
```

Validates Merkle root.

#### get_zeris_state

```rust
pub fn get_zeris_state(&self) -> Result<ZerisState, Box<dyn std::error::Error>>
```

Retrieves current state.

**Returns:** Current Zeris state

## Solana Program Instructions

### submit_proof

**Instruction Code:** 0

**Accounts:**
- zeris_state (writable)
- signer (writable, signer)
- system_program (readonly)

**Data:**
- instruction_code: u8 = 0
- proof_bytes: Vec<u8>
- public_signals: Vec<u8>

### verify_proof

**Instruction Code:** 1

**Accounts:**
- zeris_state (writable)

**Data:**
- instruction_code: u8 = 1
- proof_hash: [u8; 32]

### validate_merkle_root

**Instruction Code:** 2

**Accounts:**
- zeris_state (readonly)

**Data:**
- instruction_code: u8 = 2
- root_hash: [u8; 32]

## Events

### ProofSubmitted

```rust
#[event]
pub struct ProofSubmitted {
    pub proof_hash: [u8; 32],
    pub submitter: Pubkey,
}
```

### ProofVerified

```rust
#[event]
pub struct ProofVerified {
    pub proof_hash: [u8; 32],
    pub is_valid: bool,
}
```

### MerkleRootUpdated

```rust
#[event]
pub struct MerkleRootUpdated {
    pub new_root: [u8; 32],
}
```

### MerkleRootValidated

```rust
#[event]
pub struct MerkleRootValidated {
    pub root_hash: [u8; 32],
    pub is_valid: bool,
}
```

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 0 | InvalidProofFormat | Proof data is malformed |
| 1 | ProofVerificationFailed | Proof verification failed |
| 2 | InvalidProofHash | Proof hash doesn't match stored |
| 3 | MerkleRootMismatch | Merkle root validation failed |
| 4 | Unauthorized | Unauthorized access |
| 5 | InvalidPublicSignals | Public signals are invalid |