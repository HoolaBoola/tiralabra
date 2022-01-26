#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

RUSTFLAGS="-Z instrument-coverage" cargo +nightly test
grcov $SCRIPT_DIR --binary-path $SCRIPT_DIR/target/debug/ -s . -t html --branch --ignore-not-existing -o $SCRIPT_DIR/coverage/

