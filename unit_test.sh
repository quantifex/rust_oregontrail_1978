#!/bin/bash

rm -rf coverage

export RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly test
# Print Code Coverage to the console
echo -n "Total Coverage %: "
grcov . --binary-path target/debug -s . --ignore="/*" -t covdir --ignore-not-existing | jq '.coveragePercent'
echo
# Generate Code Coverage web report
grcov . --binary-path target/debug -s . --ignore="/*" -t html --branch --ignore-not-existing -o ./coverage/
# Generate Code Coverage for codecov badge
grcov . --binary-path target/debug -s . --ignore="/*" -t lcov --branch --ignore-not-existing -o ./coverage/lcov.info
