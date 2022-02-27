@rraval's reusable GitHub workflows
===================================

Repository for common workflows so I only have to bother getting it right once.

## [.github/workflows/rust_dev.yml](.github/workflows/rust_dev.yml)

A comprehensive set of Rust checks intended to catch errors during development:

- Runs the test suite with [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
- Collects source-based coverage with [`grcov`](https://github.com/mozilla/grcov) and uploads it to [Coveralls](https://coveralls.io/).
- Reports errors with [`cargo check`](https://doc.rust-lang.org/cargo/commands/cargo-check.html).
- Lints with [`clippy`](https://github.com/rust-lang/rust-clippy).
- Checks formatting with [`rustfmt`](https://github.com/rust-lang/rustfmt).

### Usage

Add a workflow file to your repository like `.github/workflows/dev.yml` with the following contents:

```
name: Dev
on:
  push:
    branches-ignore:
      - "dependabot/**"
  pull_request:
    types: [opened, reopened]
jobs:
  all:
    uses: rraval/workflows/.github/workflows/rust_dev.yml@v1
```

### Demo

See [.github/workflows/rust_dev.example.yml](.github/workflows/rust_dev.example.yml) for a demo that checks a toy Rust crate from this repository.

- [A commit passing all checks](https://github.com/rraval/workflows/actions/runs/1702145604)
- [A pull request that fails most checks](https://github.com/rraval/workflows/actions/runs/1702179404)
- [Coverage for toy example](https://coveralls.io/builds/45655806)
- [Coverage for a real project: `git-nomad`](https://coveralls.io/builds/45651584)

## [.github/workflows/rust_publish.yml](.github/workflows/rust_publish.yml)

Publishes the crate to [crates.io](https://crates.io) with [`cargo publish`](https://doc.rust-lang.org/cargo/commands/cargo-publish.html).

### Usage

Navigate to <https://crates.io/settings/tokens> and generate a new token specific to your repository.

Follow the [GitHub instructions for creating a repository secret](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository) and create a secret named `CRATES_IO_TOKEN` with the value from <https://crates.io/settings/tokens>.

Add a workflow file to your repository like `.github/workflows/publish.yml` with the following contents:

```
name: Publish
on:
  release:
    types: [published]
jobs:
  all:
    uses: rraval/workflows/.github/workflows/rust_publish.yml@v1
    secrets:
      CRATES_IO_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}
```

### Demo

See [.github/workflows/rust_publish.example.yml](.github/workflows/rust_publish.example.yml) for a demo that publishes a toy Rust crate from this repository.

- [Automatic `cargo publish`](https://github.com/rraval/workflows/actions/runs/1702147499) for <https://crates.io/crates/rraval-workflows>

## [.github/workflows/rust_release_binary.yml](.github/workflows/rust_release_binary.yml)

Builds Rust binaries (Linux and Mac OS X) and uploads them as artifacts to a [GitHub release](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository).

### Usage

Add a workflow file to your repository like `.github/workflows/release.yml` with the following contents, replacing `<NAME-OF-YOUR-CRATE-BINARY>` with the binary to build [as specified in the `Cargo.toml`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries) (if you're not doing anything fancy, this is usually the same as the [Cargo package name](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field)).

```
name: Release
on:
  release:
    types: [published]
jobs:
  main:
    uses: rraval/workflows/.github/workflows/rust_release_binary.yml@v1
    with:
      CARGO_BINARY_NAME: <NAME-OF-YOUR-CRATE-BINARY>
```

### Demo

See [.github/workflows/rust_release_binary.example.yml](.github/workflows/rust_release_binary.example.yml) for a demo that builds and uploads 2 binaries for a toy Rust crate from this repository.

- [Automatic `cargo build`](https://github.com/rraval/workflows/actions/runs/1702147498) of release [v1.0.0-rc2](https://github.com/rraval/workflows/releases/tag/v1.0.0-rc2).
