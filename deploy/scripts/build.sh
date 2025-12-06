#!/bin/bash

# Zeris Program Build Script
# Builds the Anchor program for deployment

set -e

echo "🔨 Building Zeris Validator Program..."

# Navigate to program directory
cd programs/zeris-validator

# Clean previous build
echo "Cleaning previous build..."
anchor clean

# Build the program
echo "Building program..."
anchor build

# Verify IDL generation
if [ ! -f "target/idl/zeris_validator.json" ]; then
    echo "❌ IDL not generated"
    exit 1
fi

echo "✅ Build completed successfully"
echo "📁 Build artifacts in: programs/zeris-validator/target/"
echo "📄 IDL file: programs/zeris-validator/target/idl/zeris_validator.json"