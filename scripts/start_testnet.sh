#!/bin/bash
# Start the testnet with 4 validators
echo "Starting Nocoin™ Testnet"
cargo run --bin validator -- --config config/testnet1.toml &
cargo run --bin validator -- --config config/testnet2.toml &
cargo run --bin validator -- --config config/testnet3.toml &
cargo run --bin validator -- --config config/testnet4.toml &
