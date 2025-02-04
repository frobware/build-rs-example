#!/usr/bin/env bash
set -eu

check_output() {
    local expected="$1"
    cargo clean
    got=$(cargo run --quiet)
    echo "expected '$expected', got '$got'"
    [[ "$got" == "$expected" ]] || exit 1
}

check_output "unknown"
export PACKAGE_VERSION=fortytwo.42.42.42
check_output "$PACKAGE_VERSION"
