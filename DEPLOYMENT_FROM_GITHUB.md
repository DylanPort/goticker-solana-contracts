# GoTicker Contract Deployment via GitHub Actions

This guide walks you through deploying the GoTicker Solana contracts using GitHub Actions for building on Linux.

## 🚀 Quick Start

### 1. Create GitHub Repository
1. Go to [GitHub.com](https://github.com) and create a new repository
2. Name: `goticker-solana-contracts`
3. Set to **Public** (required for free GitHub Actions)
4. Don't initialize with files

### 2. Push Code to GitHub
```bash
git remote add origin https://github.com/YOUR_USERNAME/goticker-solana-contracts.git
git push -u origin main
```

### 3. GitHub Actions Build
- Actions will automatically trigger and build the contracts on Linux
- Wait 5-10 minutes for completion
- Download the artifacts containing `.so` files

### 4. Deploy to Mainnet
```bash
# Extract the downloaded artifacts
# Then run:
./deploy-from-github.ps1
```

## 📋 Current Program IDs

- **GoTicker Registry**: `9AcKM9kiHLxM22V3QUY6QsxQFysWiE8oLqAUA6PrQRWB`
- **Ticker Economy**: `8UJTSRqXtgj5FNEkRB56DFGPEteNCQSED6n4pavpexqM` (new)

## 💰 Wallet Status

- **Address**: `2NamKpRryme323e5WSAPqq6K2ssLAsaEfJymYLPk9e78`
- **Balance**: 0.1 SOL (sufficient for deployment)

## 🔧 What Gets Built

The GitHub Actions workflow will:
1. Set up Ubuntu Linux environment
2. Install Rust, Solana CLI v1.18.26, and Anchor CLI v0.26.0
3. Build both contracts with proper Solana BPF/SBF compilation
4. Generate `.so` files and IDL files
5. Upload as downloadable artifacts

## ✅ Deployment Success

Once deployed, the subscription contract will be available at:
- **Program ID**: `8UJTSRqXtgj5FNEkRB56DFGPEteNCQSED6n4pavpexqM`
- **Network**: Solana Mainnet
- **Features**: Payment processing, subscriptions, revenue sharing 