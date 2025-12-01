#!/bin/bash
set -eo pipefail

# Compile
cd ../../compile;
cargo run compile rust ../test/rust/schema/ ../test/rust/src/;
cd ../test/rust;
cargo fmt;

# Test
cargo test --all-features;
