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

**Frontend (Vercel):**
1. Push your repository to GitHub.
2. Sign in to [Vercel](https://vercel.com/) and click "Add New Project".
3. Import your GitHub repository, set the root directory to `frontend`, and keep the default Vite build settings.
4. Click "Deploy". Your frontend is live!

## 🧪 Test Instructions
This contract implements 3 meaningful tests (found in `contracts/crowdfund/src/test.rs`):
1. **Contract Logic Test**: Verifies campaign initialization and successful pledges.
2. **Edge Case Test**: Attempts an invalid refund before the deadline finishes.
3. **Interaction Test**: Emulates a full lifecycle (Init -> Meet Goal -> Withdraw).

## 🔗 Links
- **Live Demo**: `[Insert Vercel Link Here]`
- **Video Walkthrough**: `[Insert Video URL Here]`

## 📸 Screenshot Placeholder
`![Screenshot](placeholder)`
