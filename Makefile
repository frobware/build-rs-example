# Detect version from git tags, fallback to "unknown".
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo "unknown")
TARBALL := build-rs-example-$(VERSION).tar.gz
TESTDIR := $(shell mktemp -d)

.PHONY: all tarball test clean

all: tarball test

tarball:
	@echo "Creating source tarball using git archive"
	git archive --format=tar.gz --prefix=build-rs-example-$(VERSION)/ -o $(TARBALL) HEAD

# Self-test: Extract, build, and check version output
test: tarball
	echo "Running tests in $(TESTDIR)" && \
	tar -xzf $(TARBALL) -C $(TESTDIR) && \
	cd $(TESTDIR)/build-rs-example-$(VERSION) && ./test.sh && ls -r $(TESTDIR)

# Clean tarball
clean:
	rm -f $(TARBALL)
