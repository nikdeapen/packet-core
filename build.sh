#!/usr/bin/env bash
set -euo pipefail

COMPILER_DIR="$HOME/src/proto-packet-compiler"
SCHEMA_DIR="$(dirname "$0")/schema"
TARGET_DIR="$(dirname "$0")/src"

cargo run --manifest-path "$COMPILER_DIR/Cargo.toml" -p proto-packet-cli -- compile rust "$SCHEMA_DIR" "$TARGET_DIR"

cd "$(dirname "$0")"
cargo fmt
cargo build
cargo test
