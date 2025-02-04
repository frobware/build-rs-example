#!/usr/bin/env bash

set -euo pipefail

got=$(PACKAGE_VERSION=42 cargo run --quiet)
echo "expected '42', got '$got'"
echo "$got" | grep -q "42"

got=$(cargo run --quiet)
echo "expected 'unknown', got '$got'"
echo "$got" | grep -q "unknown"
