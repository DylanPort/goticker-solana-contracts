# Deploy GoTicker contracts from GitHub Actions build
# Run this after downloading and extracting the artifacts from GitHub Actions

Write-Host "🚀 GoTicker Contract Deployment from GitHub Actions" -ForegroundColor Green
Write-Host ""

# Check if solana CLI is available
if (!(Get-Command "solana" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Solana CLI not found. Please ensure it's installed and in PATH." -ForegroundColor Red
    exit 1
}

# Check wallet balance
$balance = solana balance
Write-Host "💰 Current balance: $balance" -ForegroundColor Yellow

if ($balance -eq "0 SOL") {
    Write-Host "❌ No SOL balance. Please fund your wallet first." -ForegroundColor Red
    exit 1
}

# Deploy ticker economy contract (subscription system)
Write-Host "📦 Deploying Ticker Economy Contract..." -ForegroundColor Cyan
$tickerEconomyId = "8UJTSRqXtgj5FNEkRB56DFGPEteNCQSED6n4pavpexqM"

if (Test-Path "ticker_economy.so") {
    solana program deploy ticker_economy.so --program-id $tickerEconomyId
    Write-Host "✅ Ticker Economy Contract deployed!" -ForegroundColor Green
} else {
    Write-Host "❌ ticker_economy.so not found. Please extract from GitHub Actions artifacts." -ForegroundColor Red
}

Write-Host ""
Write-Host "🎉 Deployment completed!" -ForegroundColor Green
Write-Host "📍 Ticker Economy Program ID: $tickerEconomyId" -ForegroundColor Yellow 