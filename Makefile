VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TARBALL := $(shell mktemp --tmpdir build-rs-example-$(VERSION)-XXXXXXXX.tar)

all: fmt clippy build run test

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build --release

run:
	cargo run --release --quiet

# Self-test: Extract, build, and check version output.
test: clippy tarball
	./test.sh $(TARBALL) "build-rs-example $(VERSION)"

# Create a versioned tarball, excluding unnecessary files. We don't
# use `git archive` because, in development, we want to pick up
# uncommitted changes.
tarball:
	tar -cf $(TARBALL) --exclude=.git --exclude=target *

.PHONY: all fmt clippy build run tarball test
