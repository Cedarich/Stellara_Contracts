📜 Stellara AI Smart Contracts (Soroban)

Soroban smart contracts powering Stellara AI, a Web3 crypto learning and social trading platform built on the Stellar blockchain. These contracts provide decentralized services for education credentials, social rewards, messaging, and on-chain trading used by the Stellara backend and frontend applications.

This repository is intended for blockchain developers, protocol contributors, and the Stellara platform infrastructure, serving as the trust layer for learning achievements, engagement rewards, user interactions, and decentralized trading features.

## Overview

This repository contains four core smart contracts that power the Stellara ecosystem:

- **Trading Contract**: Decentralized exchange functionality for trading cryptocurrency pairs
- **Academy Contract**: Credential management for course completion and learning achievements
- **Social Rewards Contract**: Engagement tracking and reward distribution for community participation
- **Messaging Contract**: Decentralized messaging between users with read status tracking

## Project Structure

```
├── contracts/
│   ├── trading/         # DEX trading contract
│   ├── academy/         # Credential & NFT contract
│   ├── social_rewards/  # Engagement rewards contract
│   └── messaging/       # P2P messaging contract
├── shared/              # Shared utilities and types
├── Cargo.toml          # Workspace configuration
└── README.md           # This file
```

## Prerequisites

- Rust 1.70 or later
- Soroban SDK 20.5.0
- Stellar CLI tools

## Building

```bash
# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Build specific contract
cd contracts/trading
cargo build --release --target wasm32-unknown-unknown
```

## Testing

```bash
# Run all tests
cargo test --all

# Run specific contract tests
cd contracts/trading
cargo test
```

## Deployment

### Testnet Deployment

1. Set up your Stellar CLI:
```bash
stellar config network set testnet https://soroban-testnet.stellar.org
```

2. Create a network configuration:
```bash
stellar config set --scope global RPC_URL https://soroban-testnet.stellar.org
stellar config set --scope global NETWORK_PASSPHRASE "Test SDF Network ; September 2015"
```

3. Deploy contracts:
```bash
# Build WASM binaries
cargo build --release --target wasm32-unknown-unknown

# Deploy trading contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/trading_contract.wasm \
  --source account-name \
  --network testnet
```

4. Initialize contracts after deployment:
```bash
# Initialize trading contract
stellar contract invoke \
  --id CONTRACT_ADDRESS \
  --source account-name \
  --network testnet \
  -- init
```

## Contract Descriptions

### Trading Contract

Manages decentralized trading operations.

**Key Functions:**
- `init()`: Initialize the contract
- `execute_trade()`: Execute a trade on specified pair
- `get_stats()`: Retrieve trading statistics
- `get_user_trades()`: Get user's trade history

### Academy Contract

Manages educational credentials and achievements.

**Key Functions:**
- `init()`: Initialize the contract
- `issue_credential()`: Award credential to user (admin only)
- `get_user_credentials()`: Retrieve user's credentials
- `verify_credential()`: Verify a credential exists
- `get_stats()`: Retrieve credential statistics

### Social Rewards Contract

Tracks engagement and distributes rewards.

**Key Functions:**
- `init()`: Initialize the contract
- `record_engagement()`: Record user engagement activity
- `get_user_rewards()`: Get user's reward balance and tier
- `get_engagement_history()`: Get user's engagement history
- `claim_tier_reward()`: Claim rewards based on tier

### Messaging Contract

Enables decentralized P2P messaging.

**Key Functions:**
- `init()`: Initialize the contract
- `send_message()`: Send message to recipient
- `mark_as_read()`: Mark message as read
- `get_messages()`: Get user's messages (received/sent)
- `get_unread_count()`: Get count of unread messages
- `get_stats()`: Retrieve messaging statistics

## Environment Variables

For deployment, set these environment variables:

```bash
# Stellar account secret key
export STELLAR_SECRET_KEY="your-secret-key"

# Network configuration (testnet by default)
export SOROBAN_NETWORK="testnet"
export SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"
```


🛠 Build Contracts
bash
cargo build --release --target wasm32-unknown-unknown
Build a single contract:

bash
cd contracts/trading
cargo build --release --target wasm32-unknown-unknown

🧪 Testing
cargo test --all
Single contract:

bash
cd contracts/academy
cargo test

🌐 Network Configuration (Testnet Recommended)
bash
stellar config network set testnet https://soroban-testnet.stellar.org
stellar config set --scope global RPC_URL https://soroban-testnet.stellar.org
stellar config set --scope global NETWORK_PASSPHRASE "Test SDF Network ; September 2015"

🚀 Deployment (Testnet)
bash
Copy code
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/trading_contract.wasm \
  --source account-name \
  --network testnet
  
## Security Considerations
- All contracts implement authentication via `require_auth()`
- Admin functions are protected with address verification
- Contract storage uses instance storage for state management
- Consider implementing upgradeable proxy patterns for future updates

  Ecosystem Repositories

🌐 Frontend (Next.js): https://github.com/Dev-shamoo/Stellara_Ai
⚙ Backend (NestJS): https://github.com/shamoo53/Stellara_Ai_backend
⭐ Stellar Docs: https://developers.stellar.org/docs/smart-contracts/soroban/

## Contributing
🤝 Contributing-
The first step is to Fork the repository
then you Create a feature branch
Commit your changes
git pull latest changes to avoid conflicts
Submit a pull request 
Issues and feature requests are welcome.

When adding new features:

1. Create a new function in the appropriate contract
2. Add corresponding tests
3. Update this README with new function documentation
4. Ensure all tests pass before submitting




