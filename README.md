# GoTicker Solana Contracts

This repository contains the Solana smart contracts for the GoTicker platform.

## Contracts

### 1. GoTicker Registry Contract
- **Program ID**: `9AcKM9kiHLxM22V3QUY6QsxQFysWiE8oLqAUA6PrQRWB`
- **Description**: Main contract for ticker registration and trading

### 2. Ticker Economy Contract  
- **Program ID**: `6qNwVB7W2Mgb2AMB74cxKz9u5q3nf3dyPndHNN2y3Hna`
- **Description**: Subscription and payment processing system

## Development

### Prerequisites
- Rust 1.87+
- Solana CLI 1.18.26
- Anchor CLI 0.26.0

### Building
```bash
anchor build
```

### Testing
```bash
anchor test
```

### Deployment
The contracts are built automatically via GitHub Actions on push to main branch.

## Architecture

The GoTicker platform uses a two-contract architecture:
1. **Registry Contract**: Handles ticker registration, trading, and core functionality
2. **Economy Contract**: Manages subscriptions, payments, and revenue sharing

## License

This project is proprietary software for the GoTicker platform. 