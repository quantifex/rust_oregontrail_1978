#!/bin/bash -e

rm -rf coverage
rm default.profraw

export RUSTFLAGS="-Zinstrument-coverage"
export CARGO_INCREMENTAL=0
cargo +nightly test

# Generate Code Coverage (in lcov for lcov and codecov usage)
mkdir -p ./coverage
grcov . --binary-path target/debug -s . --ignore="/*" -t lcov --branch --ignore-not-existing -o ./coverage/lcov.info

if [ $(command -v genhtml) ]; then
    genhtml ./coverage/lcov.info --output-directory coverage
fi
