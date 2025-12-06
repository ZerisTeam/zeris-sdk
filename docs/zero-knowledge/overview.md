# Zero-Knowledge Proofs Overview

## What are Zero-Knowledge Proofs?

Zero-Knowledge Proofs (ZKPs) are cryptographic protocols that allow one party (the prover) to prove to another party (the verifier) that a statement is true, without revealing any information beyond the validity of the statement itself.

## Zeris ZK Architecture

Zeris implements a comprehensive ZK-verification system on Solana, supporting multiple proving systems through an extensible trait-based architecture.

### Supported Proving Systems

- **Groth16**: Efficient SNARK for general computations
- **Plonk**: Universal and updatable SNARK
- **Bulletproofs**: Range proofs and general statements

### Core Components

```
Prover → Proof Generation → Client Submission → On-Chain Verification → State Update
    ↓           ↓               ↓                   ↓              ↓
Circuit   Trusted Setup     RPC/WebSocket      Solana Program   Merkle Tree
```

## Mathematical Foundation

### Groth16 Protocol

Given a QAP (Quadratic Arithmetic Program) representation of a circuit C:

- **Setup**: Generate trusted reference string (CRS)
- **Prove**: Create proof π for witness w satisfying C(w) = 0
- **Verify**: Check pairing equations without revealing w

The verification equation for Groth16 is:

```
e(π_A, π_B) * e(π_C, g2) = e(α * β + ∑(γ * w_i) * δ, g2)
```

Where:
- π = (π_A, π_B, π_C) is the proof
- α, β, γ, δ are CRS elements
- w_i are public inputs

### Merkle Tree Integrity

Zeris maintains a Merkle tree of verified proofs:

```
Root = H(H(H(Proof1) || H(Proof2)) || H(H(Proof3) || H(Proof4)))
```

Each new proof updates the tree, ensuring cryptographic integrity of the proof sequence.

## Security Considerations

- **Trusted Setup**: Requires secure ceremony for CRS generation
- **Circuit Privacy**: Proofs reveal only statement validity
- **On-Chain Verification**: Prevents invalid proof acceptance
- **Merkle Roots**: Enable efficient proof sequence validation

## Performance Characteristics

- **Proof Size**: ~128 bytes (Groth16)
- **Verification Time**: ~1.8ms on-chain
- **Gas Cost**: ~50k compute units per verification