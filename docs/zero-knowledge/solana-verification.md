# Solana ZK-Verification

## On-Chain Architecture

### Program Structure

```
zeris-validator/
├── lib.rs              # Program entry point
├── state.rs            # Account definitions
├── instructions/       # Instruction handlers
│   ├── submit_proof.rs
│   ├── verify_proof.rs
│   └── validate_merkle_root.rs
├── errors.rs           # Custom error codes
└── utils.rs            # Verification utilities
```

### State Management

```rust
#[account]
pub struct ZerisState {
    pub last_proof_hash: [u8; 32],    // Latest verified proof
    pub merkle_root: [u8; 32],        // Current Merkle root
    pub proof_count: u64,             // Total proofs verified
    pub authority: Pubkey,            // Program authority
    pub bump: u8,                     // PDA bump
}
```

### Syscall Usage

Zeris leverages Solana's cryptographic syscalls for efficient verification:

#### SHA-256 Hashing

```rust
use solana_program::hash::hash;

// Compute proof hash
let proof_hash = hash(&proof_bytes).to_bytes();
```

#### Elliptic Curve Operations

```rust
use solana_program::sysvar::instructions;
use solana_program::ed25519_program;

// Verify Ed25519 signatures
let ix = instructions::load_current_index_checked(&instructions_sysvar)?;
```

#### alt_bn128 for Pairing

```rust
use solana_program::alt_bn128;

// Perform pairing check for Groth16
let pairing_result = alt_bn128::pairing(&point1, &point2)?;
```

## Compute Optimization

### Instruction Processing

- **Proof Parsing**: O(n) where n is proof size
- **Verification**: O(1) for most proving systems
- **Merkle Update**: O(log n) for tree operations
- **State Update**: O(1) account writes

### Memory Management

- **Stack Frames**: Limited to 4KB per instruction
- **Heap Allocation**: Avoided for performance
- **Account Data**: Pre-allocated with fixed sizes

## Security Features

### Access Control

```rust
#[derive(Accounts)]
pub struct SubmitProof<'info> {
    #[account(
        mut,
        seeds = [b"zeris-state"],
        bump = zeris_state.bump
    )]
    pub zeris_state: Account<'info, ZerisState>,

    #[account(mut)]
    pub signer: Signer<'info>,
}
```

### Input Validation

```rust
require!(proof_bytes.len() > 0, ZerisError::InvalidProofFormat);
require!(public_signals.len() <= MAX_PUBLIC_SIGNALS, ZerisError::InvalidPublicSignals);
```

### Reentrancy Protection

All state modifications happen atomically within single instructions.

## Event System

### Emitted Events

```rust
#[event]
pub struct ProofSubmitted {
    pub proof_hash: [u8; 32],
    pub submitter: Pubkey,
}

#[event]
pub struct ProofVerified {
    pub proof_hash: [u8; 32],
    pub is_valid: bool,
}

#[event]
pub struct MerkleRootUpdated {
    pub new_root: [u8; 32],
}
```

### Event Consumption

```typescript
// Listen for events
connection.onLogs(programId, (logs) => {
    // Parse event logs
    const events = parseEventLogs(logs.logs);
    events.forEach(event => {
        handleEvent(event);
    });
});
```

## Performance Benchmarks

| Operation | Compute Units | Time (ms) | Cost (SOL) |
|-----------|---------------|-----------|------------|
| submit_proof | 45,000 | 2.5 | 0.0012 |
| verify_proof | 28,000 | 1.8 | 0.0008 |
| validate_merkle_root | 15,000 | 1.2 | 0.0005 |

## Deployment Strategy

### Devnet Testing

```bash
# Build and deploy
anchor build
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment

```bash
# Use deployment scripts
./deploy/scripts/deploy-mainnet.sh
```

### Program Upgrade

Zeris supports program upgrades through Anchor's upgradeable loader:

```rust
// Upgrade instruction
pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> Result<()> {
    // Implementation
}
```