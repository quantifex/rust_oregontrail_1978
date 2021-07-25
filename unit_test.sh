#!/bin/bash -e

rm -rf coverage
rm -f default.profraw

export RUSTFLAGS="-Zinstrument-coverage"
export CARGO_INCREMENTAL=0
cargo +nightly test

# Generate Code Coverage (in lcov for genhtml and codecov usage)
mkdir -p ./coverage
grcov . --binary-path target/debug -s . --ignore="/*" --ignore="src/main.rs" -t lcov --branch --ignore-not-existing -o ./coverage/lcov.info

# Only run genhtml if it's installed (it's not included in the raw rust_devtest container)
if [ $(command -v genhtml) ]; then
    genhtml ./coverage/lcov.info --output-directory coverage
fi
