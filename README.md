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
- The value of `$PACKAGE_VERSION` (e.g. `42.42.42`) when set and no Git repository is present
- `unknown` as a final fallback

## Usage

Inside a Git repository, Git is the single source of truth - the version will always come from Git regardless of environment variables:
```bash
$ cargo run
   Compiling build-rs-example v0.1.0 (/home/aim/src/github.com/frobware/r/build-rs-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/build-rs-example`
build-rs-example 0.0.1-1-g0e1958e (2024-04-05T20:26:13Z) [rustc 1.84.0 (9fc6b4312 2025-01-07) (Fedora 1.84.0-3.fc41)]
```

Outside a Git repository (e.g. when building from a tarball), `PACKAGE_VERSION` is used:
```bash
$ PACKAGE_VERSION=1.2.3 cargo run --quiet
build-rs-example 1.2.3 (2024-04-05T20:32:27Z) [rustc 1.84.0 (9fc6b4312 2025-01-07) (Fedora 1.84.0-3.fc41)]
```

## Testing

Run `make test` to see the `PACKAGE_VERSION` behaviour demonstrated with a tarball build outside the repository.
