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

<details>
<summary>Usage Guide</summary>

Add a workflow file to your repository like `.github/workflows/dev.yml` with the following contents:

```
name: Dev
on: [push, pull_request]
jobs:
  all:
    uses: rraval/workflows/.github/workflows/rust_dev.yml@master
```

</details>
