# Zeris Node.js TypeScript Example

This example demonstrates how to use the Zeris TypeScript SDK to submit zero-knowledge proofs to the Solana blockchain.

## Prerequisites

- Node.js 18+
- A Solana keypair with some DEV tokens

## Setup

```bash
# Install dependencies
npm install

# Build the example
npm run build
```

## Generate Keypair (if needed)

The example will generate a new keypair if `keypair.json` doesn't exist. For testing, you'll need some DEV tokens:

```bash
# Get DEV tokens
solana airdrop 2 <your-public-key>
```

## Generate Proof (Optional)

For a real example, generate a proof using snarkjs:

```bash
# Install snarkjs
npm install -g snarkjs

# Create a simple circuit
echo 'pragma circom 2.0.0;
template Multiplier() {
    signal input a;
    signal input b;
    signal output c;
    c <== a * b;
}
component main = Multiplier();' > circuit.circom

# Compile and setup (simplified)
circom circuit.circom --wasm --r1cs
snarkjs groth16 setup circuit.r1cs pot12_final.ptau circuit_final.zkey
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json

# Create witness and prove
# ... (additional steps for full proof generation)
```

## Run the Example

```bash
npm start
```

This will:
1. Load or generate a keypair
2. Build mock proof data
3. Submit the proof to the Zeris program
4. Wait for confirmation
5. Display the updated state
6. Listen for events

## Expected Output

```
Zeris Node.js TypeScript Example
Public key: <your-public-key>
Building proof data...
Submitting proof...
✅ Proof submitted successfully!
Transaction signature: <transaction-signature>
✅ Transaction confirmed
Fetching current state...
📊 Current state:
   Last Proof Hash: <hash>
   Merkle Root: <root>
   Proof Count: 1
   Authority: <authority>
Listening for events...
📡 Event: { type: 'ProofSubmitted', proofHash: <hash>, submitter: <pubkey> }
📡 Event: { type: 'ProofVerified', proofHash: <hash>, isValid: true }
📡 Event: { type: 'MerkleRootUpdated', newRoot: <root> }
```