#!/usr/bin/env bash

set -e

echo "Running format check..."
cargo fmt --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Running tests..."
cargo test

echo "Building release with SQLx offline..."
SQLX_OFFLINE=true cargo build --release

echo "CI PASSED"