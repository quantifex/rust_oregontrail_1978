#!/bin/bash

rm -rf coverage

export RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly test
# Print Code Coverage to the console
echo -n "Total Coverage %: "
grcov . --binary-path target/debug -s . -t covdir --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" | jq '.coveragePercent'
echo
# Generate Code Coverage web report
grcov . --binary-path target/debug -s . -t html --branch --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" -o ./coverage/
# Generate Code Coverage for codecov badge
grcov . --binary-path target/debug -s . -t lcov --branch --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" -o ./coverage/lcov.info
