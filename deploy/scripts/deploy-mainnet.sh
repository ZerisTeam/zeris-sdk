#!/bin/bash

# Zeris Mainnet Deployment Script
# Deploys the program to Solana Mainnet

set -e

echo "🚀 Deploying Zeris to Mainnet..."
echo "⚠️  WARNING: This will deploy to MAINNET!"
echo "Please ensure you have:"
echo "  - Sufficient SOL for deployment (~1-2 SOL)"
echo "  - Verified program code"
echo "  - Run comprehensive tests"

read -p "Are you sure you want to continue? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "Deployment cancelled"
    exit 0
fi

# Check wallet balance
BALANCE=$(solana balance | awk '{print $1}')
REQUIRED=2.0

if (( $(echo "$BALANCE < $REQUIRED" | bc -l) )); then
    echo "❌ Insufficient balance: $BALANCE SOL (need $REQUIRED SOL)"
    exit 1
fi

echo "💰 Wallet balance: $BALANCE SOL"

# Set cluster to mainnet
echo "Setting cluster to mainnet..."
solana config set --url mainnet-beta

# Navigate to program directory
cd programs/zeris-validator

# Deploy the program
echo "Deploying to mainnet..."
PROGRAM_ID=$(anchor deploy --provider.cluster mainnet | grep "Program Id:" | awk '{print $3}')

if [ -z "$PROGRAM_ID" ]; then
    echo "❌ Deployment failed"
    exit 1
fi

echo "✅ Mainnet deployment successful!"
echo "📋 Program ID: $PROGRAM_ID"
echo "🌐 Mainnet Explorer: https://explorer.solana.com/address/$PROGRAM_ID"

# Update Anchor.toml with deployed program ID
sed -i "s/zeris_validator = \"ZER1S11111111111111111111111111111111111111\"/zeris_validator = \"$PROGRAM_ID\"/" Anchor.toml

echo "📝 Updated Anchor.toml with mainnet program ID"
echo "🎯 Zeris is now live on mainnet!"

# Update documentation
echo "📚 Updating documentation..."
cd ../..
sed -i "s/- Mainnet: To be filled by user after deployment/- Mainnet: $PROGRAM_ID/" docs/README.md

echo "📖 Documentation updated with mainnet program ID"