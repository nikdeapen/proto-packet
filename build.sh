#!/bin/bash
set -e;

rm -rf proto-packet-test/src/;
mkdir proto-packet-test/src/;
touch proto-packet-test/src/lib.rs;
cargo run compile rust proto-packet-test/schema/ proto-packet-test/src/;
cargo fmt;
cargo test --all-features;
