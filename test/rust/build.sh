#!/bin/bash
set -euo pipefail

# Compile Schema
cd ../../compile;
cargo run compile rust ../test/rust/schema/ ../test/rust/src/;
cd ../test/rust;
cargo fmt;

# Test
cargo test --all-features;
