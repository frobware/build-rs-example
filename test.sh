#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: ${0##*/} <tarball> <expected-version>"
    exit 1
fi

tarball="$1"
expected_version="$2"

if [[ ! -f "$tarball" ]]; then
    echo "Error: '$tarball' not found." >&2
    exit 1
fi

testdir=$(mktemp -d)
trap 'rm -rf "$testdir"' EXIT

tar --extract --gzip --file "$tarball" --directory "$testdir"
cd "$testdir"

# Force a rebuild; a precautionary measure to prevent tarballs from
# containing stale build artefacts.
cargo clean

if ! got=$(cargo run); then
    exit 1;
fi

echo "$got" | grep -q "$expected_version" || {
    echo "Error: Version mismatch!" >&2
    echo "    Got:      '$got'" >&2
    echo "    Expected: '$expected_version'" >&2
    exit 1
}

echo "Got:     '$got'"
echo "Matched: '$expected_version'"
