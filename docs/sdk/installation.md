# Installation Guide

## Prerequisites

### System Requirements

- **Rust**: 1.70.0 or later
- **Node.js**: 18.0.0 or later
- **Anchor**: 0.28.0 or later
- **Solana CLI**: 1.16.0 or later

### Installation Commands

#### macOS/Linux

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (using nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.28.0
avm use 0.28.0
```

#### Windows

```powershell
# Install Rust
winget install --id Rustlang.Rustup

# Install Node.js
winget install OpenJS.NodeJS

# Install Solana CLI
winget install solana-labs.solana

# Install Anchor (requires WSL for full functionality)
# Follow Anchor documentation for Windows setup
```

## SDK Installation

### TypeScript SDK

```bash
# Install from npm
npm install @zeris/sdk

# Or install from source
cd sdk/typescript
npm install
npm run build
npm link
```

### Rust SDK

```toml
# Add to Cargo.toml
[dependencies]
zeris-sdk = "0.1.0"
```

```bash
# Install from source
cd sdk/rust
cargo build --release
```

## Program Development Setup

### Clone Repository

```bash
git clone https://github.com/your-org/zeris-monorepo.git
cd zeris-monorepo
```

### Install Dependencies

```bash
# Install Node.js dependencies
npm install

# Install Rust dependencies
cd programs/zeris-validator
cargo build

# Install Anchor dependencies
anchor install
```

### Environment Setup

```bash
# Configure Solana CLI
solana config set --url devnet

# Generate keypair
solana-keygen new --outfile ~/.config/solana/id.json

# Airdrop SOL for testing
solana airdrop 2
```

## Development Workflow

### Building the Program

```bash
cd programs/zeris-validator

# Build with Anchor
anchor build

# Or build with Cargo
cargo build-bpf
```

### Running Tests

```bash
# Run Anchor tests
anchor test

# Run TypeScript tests
cd ../../sdk/typescript
npm test

# Run Rust tests
cd ../rust
cargo test
```

### Local Development

```bash
# Start local validator
solana-test-validator

# Deploy to localnet
anchor deploy --provider.cluster localnet

# Run examples
cd examples/node-ts-example
npm install
npm start
```

## Troubleshooting

### Common Issues

#### Anchor Build Failures

```bash
# Clear Anchor cache
anchor clean

# Rebuild
anchor build
```

#### Solana CLI Issues

```bash
# Check version
solana --version

# Update CLI
solana-install update
```

#### Node.js Compatibility

```bash
# Check Node version
node --version

# Update npm
npm install -g npm@latest
```

### Getting Help

- **Documentation**: [docs.zeris.io](https://docs.zeris.io)
- **Issues**: [GitHub Issues](https://github.com/your-org/zeris-monorepo/issues)
- **Discord**: [Zeris Community](https://discord.gg/zeris)