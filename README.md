# SorobanFundMe - Stellar Journey to Mastery 🚀

A decentralized crowdfunding dApp built on Stellar using Soroban smart contracts. This project demonstrates core DeFi mechanics like conditional fund locking, permissionless interactions, state caching, and deadline enforcement.

## 🌟 Features
- **Create Campaign**: Users establish a funding target and time deadline.
- **Pledge Funds**: Backers securely lock XLM into the contract.
- **Milestone Withdrawal**: The creator can withdraw the total pool *only* if the target is met *and* the deadline has passed.
- **Refunds**: Backers can reclaim their XLM if the campaign deadline passes without hitting the target.
- **State Caching**: The frontend utilizes smart caching to remember the user's latest pledged totals instantly.
- **Responsive UI**: A sleek, dark-mode Web3 interface with transaction lifecycle feedback.

## 🛠️ Tech Stack
- **Smart Contract**: Rust, Soroban SDK
- **Frontend**: React, Vite, Vanilla CSS
- **Integration**: `@stellar/stellar-sdk`, `@stellar/freighter-api`

## 📦 Installation & Setup

### 1. Smart Contract
1. Install Rust, target `wasm32-unknown-unknown`, and the `stellar-cli`.
2. Build the contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
3. Run the test suite:
   ```bash
   cargo test
   ```

### 2. Frontend
1. Navigate to the frontend directory:
   ```bash
   cd frontend
   npm install
   ```
2. Start the local development server:
   ```bash
   npm run dev
   ```

## 🚀 Deployment Guide
**Smart Contract (Stellar Testnet):**
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_crowdfund_contract.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```



## 🧪 Test Instructions
This contract implements 3 meaningful tests (found in `contracts/crowdfund/src/test.rs`):
1. **Contract Logic Test**: Verifies campaign initialization and successful pledges.
2. **Edge Case Test**: Attempts an invalid refund before the deadline finishes.
3. **Interaction Test**: Emulates a full lifecycle (Init -> Meet Goal -> Withdraw).

## 🔗 Links
app link: https://token-vesting-contract-test-d-iqz0pwss9-ranit-sarkars-projects.vercel.app/
- **Video Walkthrough**: `[Insert Video URL Here]`

## 📸 Screenshot Placeholder
<img width="815" height="822" alt="Screenshot 2026-04-24 025408" src="https://github.com/user-attachments/assets/72fc8fb0-fb38-4761-9a14-1b3abaef300e" />

_____________________________________________________________________________________________________________________________________________

<img width="884" height="753" alt="Screenshot 2026-04-24 025548" src="https://github.com/user-attachments/assets/ce46265d-5691-4cdb-8a71-503695d0dfe8" />

