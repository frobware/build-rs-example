# Rust `build.rs` Example

This repository demonstrates how to use Rust's `build.rs` system to generate **version information at compile time**. The example extracts version details from **Cargo.toml**, **Git commit hashes**, and **environment variables**, ensuring correct versioning for both **development and distribution builds**.

## How it Works

The `build.rs` script runs **before compilation** and sets a **single authoritative environment variable**:

- **`BUILD_VERSION`** → Contains all versioning details in a **Rust-like format**:
  ```
  <Cargo package version> (<Git commit hash> <build timestamp>) <Rust compiler version>
  ```

This version information is embedded into the final binary and displayed when run.

## Versioning Behavior
The output format follows Rust's versioning style (`rustc --version`), structured as:

```
<version> (<git commit> <build date>) <rustc version>
```

### Example Outputs
#### When built inside a Git repository

```sh
$ cargo run --quiet
build-rs-example 0.4.0-pre (e5dbc1d284-dirty 2025-02-15T22:35:44Z) rustc 1.84.1 (e71f9a9a9 2025-01-27)
```
- **Git commit hash included**: `e5dbc1d284`
- **Dirty flag (`-dirty`)** if uncommitted changes exist
- **Build timestamp**: `2025-02-15T22:09:48Z`
- **Rust compiler version**: `rustc 1.84.1 (e71f9a9a9 2025-01-27)`

#### When built from a tarball (no Git available)

```sh
$ make test
Got:     'build-rs-example 0.4.0-pre (2025-02-15T22:55:37Z) rustc 1.84.1 (e71f9a9a9 2025-01-27)'
Matched: 'build-rs-example 0.4.0-pre'
```

- **Git hash omitted** (not available)
- **Build timestamp remains**
- **Rust compiler version remains**
