# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Backwards compatibility is defined as:

- New versions of workflows should keep working with configurations created for older versions.
- New versions of workflows should continue producing artifacts for environments that old workflows used to produce.

## [Unreleased]

## [v1.0.0]

An initial release based on Rust workflows in https://github.com/rraval/git-nomad.

### Added

- `.github/workflows/rust_dev.yml`: A comprehensive set of Rust checks intended to catch errors during development.
- `.github/workflows/rust_publish.yml`: Publishes the crate to [crates.io](https://crates.io) with [`cargo publish`](https://doc.rust-lang.org/cargo/commands/cargo-publish.html).
- `.github/workflows/rust_release_binary.yml`: Builds Rust binaries (Linux and Mac OS X) and uploads them as artifacts to a [GitHub release](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository).
