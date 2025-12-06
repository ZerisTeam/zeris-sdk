#!/bin/bash

# Zeris Devnet Deployment Script
# Deploys the program to Solana Devnet

set -e

echo "🚀 Deploying Zeris to Devnet..."

# Check if wallet is configured
if [ ! -f ~/.config/solana/id.json ]; then
    echo "❌ Solana wallet not found. Run: solana-keygen new"
    exit 1
fi

# Set cluster to devnet
echo "Setting cluster to devnet..."
solana config set --url devnet

# Check wallet balance
BALANCE=$(solana balance | awk '{print $1}')
if (( $(echo "$BALANCE < 1.0" | bc -l) )); then
    echo "⚠️  Low balance: $BALANCE SOL. Requesting airdrop..."
    solana airdrop 2
fi

# Navigate to program directory
cd programs/zeris-validator

# Deploy the program
echo "Deploying program..."
PROGRAM_ID=$(anchor deploy --provider.cluster devnet | grep "Program Id:" | awk '{print $3}')

if [ -z "$PROGRAM_ID" ]; then
    echo "❌ Deployment failed"
    exit 1
fi

echo "✅ Deployment successful!"
echo "📋 Program ID: $PROGRAM_ID"
echo "🌐 Devnet Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"

# Update Anchor.toml with deployed program ID
sed -i "s/zeris_validator = \"ZER1S11111111111111111111111111111111111111\"/zeris_validator = \"$PROGRAM_ID\"/" Anchor.toml

echo "📝 Updated Anchor.toml with deployed program ID"
echo "🎯 Ready for testing on devnet"