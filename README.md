# Rust `build.rs` Example

This repository demonstrates how to use Rust's `build.rs` system to generate **version information at compile time**. The example extracts version details from **Cargo.toml**, **Git commit hashes**, and **environment variables**, ensuring correct versioning for both **development and distribution builds**.

## How it Works

The `build.rs` script runs **before compilation** and sets a **single authoritative environment variable**:

- **`BUILD_VERSION`** → Contains all versioning details in a **Rust-like format**:
  ```
  <Cargo package version> (<Git commit hash> <build timestamp>) <Rust compiler version>
  ```

This version information is embedded into the final binary and displayed when run.

## Versioning Behaviour
The output format follows Rust's versioning style (`rustc --version`), structured as:

```
<version> (<git commit> <build date>) <rustc version>
```

### Example Outputs
#### When built inside a Git repository

```sh
$ cargo run --quiet --bin main1
main1 0.0.15-pre (f4a3c814d9-dirty 2025-03-07T19:38:55Z) rustc 1.84.1 (e71f9a9a9 2025-01-27)
```
- **Git commit hash included**: `e5dbc1d284`
- **Dirty flag (`-dirty`)** if uncommitted changes exist
- **Build timestamp**: `2025-02-15T22:09:48Z`
- **Rust compiler version**: `rustc 1.84.1 (e71f9a9a9 2025-01-27)`

#### When built from a tarball (i.e., no Git repository present)

```sh
$ make test
Got:     'main1 0.0.15-pre (2025-03-07T19:39:13Z) rustc 1.84.1 (e71f9a9a9 2025-01-27)'
Matched: 'main1 0.0.15-pre'

...

Got:     'main2 0.0.15-pre (2025-03-07T19:39:14Z) rustc 1.84.1 (e71f9a9a9 2025-01-27)'
Matched: 'main2 0.0.15-pre'
```

- **Git hash omitted** (not available)
- **Build timestamp remains**
- **Rust compiler version remains**
