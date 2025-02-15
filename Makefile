VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
TARBALL := $(shell mktemp --tmpdir build-rs-example-$(VERSION)-XXXXXXXX.tar)
TESTDIR := $(shell mktemp -d)

run:
	cargo run --quiet

# Self-test: Extract, build, and check version output.
test: run tarball
	./test.sh $(TARBALL) "build-rs-example $(VERSION)"

# Create a versioned tarball, excluding unnecessary files. We don't
# use git archive because, in development, we want to pick up
# uncommitted changes.
tarball:
	tar -cf $(TARBALL) --exclude=.git --exclude=target *

.PHONY: run test tarball
