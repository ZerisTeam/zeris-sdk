#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "Installing TypeScript SDK..."
pushd "$ROOT/sdk/typescript"
npm install
npm run build
popd

echo "Installing Node example..."
pushd "$ROOT/examples/node-ts-example"
npm install
npm run build
popd

echo "Building Rust SDK..."
pushd "$ROOT/sdk/rust"
cargo build
popd

echo "Building Anchor program (optional; requires anchor CLI)..."
pushd "$ROOT/programs/zeris-validator"
if command -v anchor >/dev/null 2>&1; then
  anchor build
else
  echo "anchor not found; skipping"
fi
popd

echo "Installing Python requirements..."
python -m pip install -r "$ROOT/requirements.txt"
