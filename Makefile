VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TARBALL := $(shell mktemp --tmpdir build-rs-example-$(VERSION)-XXXXXXXX.tar)

all: run test

run: clippy-check
	cargo run --quiet

# Self-test: Extract, build, and check version output.
test: clippy-check tarball
	./test.sh $(TARBALL) "build-rs-example $(VERSION)"

clippy-check:
	cargo clippy -- --deny warnings

# Create a versioned tarball, excluding unnecessary files. We don't
# use `git archive` because, in development, we want to pick up
# uncommitted changes.
tarball:
	cargo clean
	tar -cf $(TARBALL) --exclude=.git --exclude=target *

.PHONY: clippy-check run tarball test
