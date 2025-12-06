# Proving Systems

## Groth16 Implementation

### Circuit Compilation

```rust
// Example circuit for a simple statement
pub struct SimpleCircuit<F: Field> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Field> ConstraintSynthesizer<F> for SimpleCircuit<F> {
    fn synthesize(&self, cs: &mut ConstraintSystem<F>) -> Result<(), SynthesisError> {
        // x * y = z
        cs.enforce(
            || "x * y = z",
            |lc| lc + self.x,
            |lc| lc + self.y,
            |lc| lc + self.z,
        );
        Ok(())
    }
}
```

### Proof Generation

```typescript
import { groth16 } from 'snarkjs';

const { proof, publicSignals } = await groth16.fullProve(
    { x: 3, y: 4 },
    'circuit.wasm',
    'circuit_final.zkey'
);
```

### On-Chain Verification

The Solana program uses alt_bn128 syscalls for efficient pairing checks:

```rust
pub fn verify_groth16_proof(
    proof: &Groth16Proof,
    public_signals: &[Fr],
    vk: &VerifyingKey,
) -> Result<bool> {
    // Perform pairing verification using alt_bn128
    let success = alt_bn128::pairing(&proof.a, &vk.alpha_g1, &proof.b, &vk.beta_g2)?;
    // Additional checks...
    Ok(success)
}
```

## Extensible Architecture

Zeris uses Rust traits for pluggable proving systems:

```rust
pub trait ProvingSystem {
    fn verify_proof(&self, proof_bytes: &[u8], public_signals: &[u8]) -> Result<bool>;
}

pub struct Groth16Verifier;
pub struct PlonkVerifier;

impl ProvingSystem for Groth16Verifier { /* ... */ }
impl ProvingSystem for PlonkVerifier { /* ... */ }
```

## Trusted Setup Requirements

### Ceremony Process

1. **Participant Registration**: Collect public keys
2. **Random Contribution**: Each participant adds entropy
3. **Verification**: Ensure contributions are valid
4. **Finalization**: Generate CRS for production use

### Security Parameters

- **128-bit security level**
- **Multi-party computation**
- **Verifiable delay functions** for timing