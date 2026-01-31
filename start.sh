#!/bin/bash

echo "========================================="
echo "Tulsa - Creator CRM System"
echo "In recognition of Roy and Leon Daley"
echo "========================================="
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed"
    echo "Please install from: https://rustup.rs"
    exit 1
fi

# Build if needed
if [ ! -f "target/debug/creator_crm" ] && [ ! -f "target/release/creator_crm" ]; then
    echo "Building project for the first time..."
    cargo build --release
fi

echo "Starting Creator CRM Server..."
echo ""
echo "API will be available at: http://127.0.0.1:3000/api"
echo "Web GUI will be available at: http://127.0.0.1:3000"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Run the server
cargo run
