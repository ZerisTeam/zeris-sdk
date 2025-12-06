#!/bin/bash

# Zeris Test Suite
# Runs comprehensive tests for the program and SDKs

set -e

echo "🧪 Running Zeris Test Suite..."

# Start local validator if not running
if ! pgrep -f "solana-test-validator" > /dev/null; then
    echo "Starting local Solana validator..."
    solana-test-validator > validator.log 2>&1 &
    VALIDATOR_PID=$!
    sleep 5
fi

# Set cluster to localnet
solana config set --url localhost

# Test program
echo "Testing Anchor program..."
cd programs/zeris-validator
anchor test

# Build and test TypeScript SDK
echo "Testing TypeScript SDK..."
cd ../../sdk/typescript
npm install
npm run build
npm test

# Build and test Rust SDK
echo "Testing Rust SDK..."
cd ../rust
cargo test

# Run examples
echo "Testing examples..."
cd ../../examples/node-ts-example
npm install
npm run build
# Note: Skip actual execution as it requires local deployment

cd ../rust-example
cargo check

echo "✅ All tests passed!"

# Cleanup
if [ ! -z "$VALIDATOR_PID" ]; then
    echo "Stopping local validator..."
    kill $VALIDATOR_PID
fi

echo "🎉 Test suite completed successfully"