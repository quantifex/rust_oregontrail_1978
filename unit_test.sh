#!/bin/bash

export RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly test
echo -n "Total Coverage %: "
grcov . --binary-path target/debug -s . -t covdir --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" | jq '.coveragePercent'
echo
grcov . --binary-path target/debug -s . -t html --branch --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" -o ./coverage/
