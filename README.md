# Rust build.rs Example

This repository demonstrates how to use Rust's `build.rs` system to generate version information at compile time. The example shows how to extract version information from Git tags, commit hashes, or environment variables, making it suitable for both development and distribution builds.

## How it works

The `build.rs` script runs before compilation and generates a `version.rs` file containing the current version string. This version information is then included in your final binary.

### Version outputs

When run, your binary will output one of:
- A Git tag (e.g. `0.1.0`) when built on a tagged commit
- A Git description (e.g. `0.1.0-5-gabcdef1`) when built after a tag
- A Git hash (e.g. `gabcdef1`) when no tags exist 
- A Git hash with dirty flag (e.g. `gabcdef1-dirty`) when uncommitted changes exist
- `uncommitted` when in a fresh Git repository
- An environment variable (e.g. `42.42.42`) when `PACKAGE_VERSION` is set and no Git repository is present
- `unknown` as a final fallback

## Usage

Inside a Git repository, Git is the single source of truth - the version will always come from Git regardless of environment variables:
```bash
cargo run
# <Outputs Git-based version>

PACKAGE_VERSION=1.2.3 cargo run
# Still outputs Git-based version
```

Outside a Git repository (e.g. when building from a tarball), `PACKAGE_VERSION` is used:
```bash
PACKAGE_VERSION=1.2.3 cargo run
# Outputs "1.2.3"
```

Run `make test` to see this behaviour demonstrated with a tarball build outside the repository.
