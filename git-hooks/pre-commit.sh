#!/bin/bash
echo "[TST] pre-commit script is running"

rust_changes=$(git diff --staged .)

if [[ -z $rust_changes ]];
then
    echo "[Rust pre-commit]: No changes in Rust dir"
    exit 0
fi

diff=$(cargo fmt --check)
if [[ -z $diff ]]; then
    echo "[Pre-commit] Format OK";
else
    echo "[Pre-commit] You need to format your code!!"
    exit 1;
fi

set -e
cargo build