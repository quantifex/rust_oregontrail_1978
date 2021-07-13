#!/bin/bash

pwd
ls -lart

RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly test
grcov . --binary-path target/debug -s . -t html --branch --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" -o ./coverage/