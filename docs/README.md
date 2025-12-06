# Zeris Documentation

**Zeris — Zero-Knowledge Resolution & Integrity System**

## Overview

Zeris is a production-grade Solana program and SDK suite for zero-knowledge proof verification and integrity management. It provides a secure, efficient, and extensible framework for integrating ZK-proofs into Solana-based applications.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Client Apps   │───▶│  Zeris Program   │───▶│   Solana Chain  │
│                 │    │   (Solana PDA)   │    │                 │
│ - TS SDK        │    │                  │    │ - Proof Storage │
│ - Rust SDK      │    │ - Proof Verifier │    │ - Merkle Roots  │
│ - CLI Tools     │    │ - State Manager  │    │ - Event Logs    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Key Features

- **High-Security ZK-Verification**: On-chain proof verification using optimized algorithms
- **Merkle Tree Integrity**: Maintains cryptographic integrity of proof sequences
- **Event-Driven Architecture**: Real-time notifications for proof submissions and verifications
- **Multi-SDK Support**: TypeScript and Rust SDKs for different use cases
- **CLI Tools**: Command-line interface for development and operations

## Getting Started

1. [Installation](sdk/installation.md)
2. [Quick Start Guide](zero-knowledge/overview.md)
3. [SDK Usage](sdk/)
4. [Deployment](deploy/)

## Contract Addresses

### Devnet
- Program ID: `ZER1S11111111111111111111111111111111111111`

### Mainnet
- Program ID: `ZER1S11111111111111111111111111111111111111`

## Documentation Sections

- [Zero-Knowledge Proofs](zero-knowledge/)
- [SDK Documentation](sdk/)
- [API Reference](sdk/api-reference.md)
- [Examples](examples/)
- [Deployment Guide](deploy/)

## Benchmarks

| Operation | Time | Cost (SOL) |
|-----------|------|------------|
| Proof Submission | ~2.5s | 0.0012 |
| Proof Verification | ~1.8s | 0.0008 |
| Merkle Validation | ~1.2s | 0.0005 |

## Contributing

See the main [README](../README.md) for contribution guidelines.