#!/usr/bin/env pwsh

# Setup script for MCP Rust Client
# This script sets up the development environment and installs dependencies

Write-Host "🦀 Setting up MCP Rust Client development environment..." -ForegroundColor Green

# Check if Rust is installed
function Test-RustInstallation {
    try {
        $rustVersion = rustc --version
        Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "❌ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
        return $false
    }
}

# Check if Cargo is installed
function Test-CargoInstallation {
    try {
        $cargoVersion = cargo --version
        Write-Host "✅ Cargo found: $cargoVersion" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Host "❌ Cargo not found. Please install Rust toolchain" -ForegroundColor Red
        return $false
    }
}

# Install Rust components
function Install-RustComponents {
    Write-Host "📦 Installing Rust components..." -ForegroundColor Blue
    
    # Install rustfmt for code formatting
    rustup component add rustfmt
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ rustfmt installed" -ForegroundColor Green
    }
    
    # Install clippy for linting
    rustup component add clippy
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ clippy installed" -ForegroundColor Green
    }
}

# Build the project
function Build-Project {
    Write-Host "🔨 Building project..." -ForegroundColor Blue
    cargo build
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Project built successfully" -ForegroundColor Green
    } else {
        Write-Host "❌ Build failed" -ForegroundColor Red
        return $false
    }
    return $true
}

# Run tests
function Test-Project {
    Write-Host "🧪 Running tests..." -ForegroundColor Blue
    cargo test
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ All tests passed" -ForegroundColor Green
    } else {
        Write-Host "⚠️ Some tests failed" -ForegroundColor Yellow
    }
}

# Run formatting check
function Test-Formatting {
    Write-Host "🎨 Checking code formatting..." -ForegroundColor Blue
    cargo fmt --check
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Code is properly formatted" -ForegroundColor Green
    } else {
        Write-Host "⚠️ Code needs formatting. Run 'cargo fmt' to fix" -ForegroundColor Yellow
    }
}

# Run clippy
function Test-Linting {
    Write-Host "🔍 Running clippy..." -ForegroundColor Blue
    cargo clippy -- -D warnings
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ No clippy warnings" -ForegroundColor Green
    } else {
        Write-Host "⚠️ Clippy found issues" -ForegroundColor Yellow
    }
}

# Main setup process
function Start-Setup {
    $rustOk = Test-RustInstallation
    $cargoOk = Test-CargoInstallation
    
    if (-not $rustOk -or -not $cargoOk) {
        Write-Host "❌ Setup failed. Please install Rust toolchain first." -ForegroundColor Red
        exit 1
    }
    
    Install-RustComponents
    
    $buildOk = Build-Project
    if (-not $buildOk) {
        Write-Host "❌ Setup failed during build." -ForegroundColor Red
        exit 1
    }
    
    Test-Project
    Test-Formatting
    Test-Linting
    
    Write-Host "`n🎉 Setup complete! You're ready to develop with MCP Rust Client." -ForegroundColor Green
    Write-Host "📝 Useful commands:" -ForegroundColor Blue
    Write-Host "  cargo build          - Build the project" -ForegroundColor Gray
    Write-Host "  cargo test           - Run tests" -ForegroundColor Gray
    Write-Host "  cargo fmt            - Format code" -ForegroundColor Gray
    Write-Host "  cargo clippy         - Run lints" -ForegroundColor Gray
    Write-Host "  cargo doc --open     - Generate and view documentation" -ForegroundColor Gray
}

# Run setup
Start-Setup

