//! `build.rs` - Generates a version string at compile time.
//!
//! ## Purpose:
//! This script extracts the version information from Git at build time
//! and writes it to `version.rs`, which is included in the final
//! binary.
//!
//! This allows the binary to display version information dynamically,
//! even when built from different commits or branches.
//!
//! ## Integration with Cargo:
//! - Cargo automatically runs `build.rs` **before compilation** if it
//!   exists.
//! - The script writes `version.rs` inside `OUT_DIR` (a Cargo build
//!   directory).
//! - `main.rs` or other sources can then
//!   `include!(concat!(env!("OUT_DIR"), "/version.rs"))`.
//!
//! ## Behaviour:
//! - If a Git tag exists, it uses:
//!   `git describe --tags --always --dirty`.
//! - If no tags exist, it falls back to the latest commit hash
//!   (`git rev-parse HEAD`).
//! - If the repository is initialised but has no commits, it returns
//!   `"uncommitted"`.
//! - If Git is unavailable (e.g., tarball builds), it checks for
//!   `PACKAGE_VERSION`.
//! - If all else fails, it returns `"unknown"`.
//!
//! ## Example Version Outputs:
//! - `0.1.0` (exactly on a tag)
//! - `0.1.0-5-gabcdef1` (5 commits after a tag, on commit `abcdef1`)
//! - `gabcdef1` (no tags, using commit hash)
//! - `gabcdef1-dirty` (repository has local uncommitted changes)
//! - `uncommitted` (repo exists but has no commits)
//! - `1.2.3` (when built from a tarball with `PACKAGE_VERSION=1.2.3`)
//! - `unknown` (fallback if all methods fail)

use std::env;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PACKAGE_VERSION");

    let version = get_git_version()
        .or_else(get_git_commit_hash) // If no tags, fallback to commit hash
        .or_else(get_git_uncommitted) // If repo exists but has no commits
        .or_else(|| env::var("PACKAGE_VERSION").ok()) // Tarball builds (no .git)
        .unwrap_or_else(|| "unknown".to_string());

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo!");
    let version_file = format!("{}/version.rs", out_dir);

    println!("Generating version.rs at: {}", version_file);

    let rustc_version = get_rustc_version()
        .unwrap_or_else(|| "".to_string());

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| {
            let secs = d.as_secs();
            let days = secs / 86400;
            let secs_remaining = secs % 86400;
            let hours = secs_remaining / 3600;
            let mins = (secs_remaining % 3600) / 60;
            let secs = secs_remaining % 60;

            let days_since_epoch = days + 719163; // Days from 0000 to 1970
            let year = (400 * days_since_epoch + 97) / 146097;
            let days_of_year = days_since_epoch - (365 * year + year/4 - year/100 + year/400);
            let month = (5 * days_of_year + 2) / 153;
            let day = days_of_year - (153 * month + 2) / 5 + 1;
            let month = month + 3;
            let year = year + (month > 12) as u64;
            let month = if month > 12 { month - 12 } else { month };

            format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                    year, month, day, hours, mins, secs)
        })
        .unwrap_or_else(|_| "unknown time".to_string());

    // Write the generated version information to `version.rs`
    if let Err(e) = fs::write(
        &version_file,
        format!(
            "pub const BUILD_VERSION: &str = \"{}\";\n\
             pub const BUILD_TOOLCHAIN_VERSION: &str = \"{}\";\n\
             pub const BUILD_DATE: &str = \"{}\";\n",
            version, rustc_version, timestamp
        ),
    ) {
        eprintln!("Failed to write version.rs: {}", e);
    }
}

/// Attempts to get the latest version from Git tags. Uses
/// `git describe --tags --always --dirty` to generate a human-friendly
/// version.
fn get_git_version() -> Option<String> {
    eprintln!("get_git_version");

    let output = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty", "--abbrev=7"])
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!("git describe failed: {:?}", output.status);
        return None;
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if version.is_empty() {
        None
    } else {
        Some(version)
    }
}

/// Fallback method: Gets the current commit hash. Uses
/// `git rev-parse HEAD` to return the latest commit hash (shortened).
fn get_git_commit_hash() -> Option<String> {
    eprintln!("get_git_commit_hash");

    let output = Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!("git rev-parse HEAD failed: {:?}", output.status);
        return None;
    }

    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hash.is_empty() {
        None
    } else {
        Some(hash)
    }
}

/// Detects an empty Git repository (no commits yet). If the repository
/// exists but has no commits, return `"uncommitted"`.
fn get_git_uncommitted() -> Option<String> {
    eprintln!("get_git_uncommitted");

    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!(
            "git rev-parse --is-inside-work-tree failed: {:?}",
            output.status
        );
        return None;
    }

    let is_git_repo = String::from_utf8_lossy(&output.stdout).trim() == "true";
    if is_git_repo {
        Some("uncommitted".to_string()) // No commits yet
    } else {
        None
    }
}

fn get_rustc_version() -> Option<String> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!("rustc --version failed: {:?}", output.status);
        return None;
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if version.is_empty() {
        None
    } else {
        Some(version)
    }
}
