VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TARBALL := $(shell mktemp --tmpdir build-rs-example-$(VERSION)-XXXXXXXX.tar)

all: fmt clippy build run test

fmt:
	cargo +nightly fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build --release

run:
	cargo run --release --quiet --bin main1
	cargo run --release --quiet --bin main2

# Self-test: Extract, build, and check version output.
test: clippy tarball
	./test.sh $(TARBALL) "main1 $(VERSION)"
	./test.sh $(TARBALL) "main2 $(VERSION)"

# Create a versioned tarball, excluding unnecessary files. We don't
# use `git archive` because, in development, we want to pick up
# uncommitted changes.
tarball:
	tar -cf $(TARBALL) --exclude=.git --exclude=target *

.PHONY: all fmt clippy build run tarball test
