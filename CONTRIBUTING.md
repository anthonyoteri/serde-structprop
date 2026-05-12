# Contributing to serde-structprop

Thank you for your interest in contributing. This document covers everything
you need to get started.

## Prerequisites

- Rust stable toolchain (minimum **1.85**) — install via [rustup](https://rustup.rs/)
- `rustfmt` and `clippy` components:
  ```
  rustup component add rustfmt clippy
  ```
- [`cocogitto`](https://docs.cocogitto.io/) for commit message validation:
  ```
  cargo install cocogitto
  ```

## Building

```
cargo build
```

## Testing

Run the full test suite, including property-based tests:

```
cargo test
```

Run a specific test file:

```
cargo test --test integration
cargo test --test prop
```

## Linting and formatting

All CI checks must pass before a PR is merged.  Run them locally with:

```
cargo fmt --check        # formatting
cargo clippy --all-targets  # lints (pedantic, zero warnings)
```

To auto-format:

```
cargo fmt
```

## Commit messages

This project enforces [Conventional Commits](https://www.conventionalcommits.org/).
Every commit message must follow the form:

```
<type>(<optional scope>): <short description>

[optional body]

[optional footer(s)]
```

Common types:

| Type | When to use |
|---|---|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only |
| `test` | Adding or correcting tests |
| `chore` | Tooling, CI, dependencies |
| `refactor` | Code change that is neither a fix nor a feature |
| `perf` | Performance improvement |

The CI pipeline runs `cog check` on every push and will reject non-conforming
commit messages.  You can validate locally before pushing:

```
cog check
```

## Submitting a pull request

1. Fork the repository and create a branch from `main`.
2. Make your changes, following the guidelines above.
3. Ensure `cargo test`, `cargo clippy --all-targets`, and `cargo fmt --check`
   all pass.
4. Open a pull request against `main` and fill in the PR template.

## Releasing

Releases are managed by maintainers.  The easiest path is the `just release`
recipe, which runs all pre-flight checks and then automates every step:

```
just release
```

This will:
1. Verify you are on `main`, the tree is clean, and local `main` is not behind `origin/main`.
2. Run `cargo test`, `cargo fmt --check`, and `cargo clippy`.
3. Run `cog bump --auto` to determine the next semver version from the commit
   history, update `Cargo.toml`, generate `CHANGELOG.md`, and create a commit
   and annotated tag.
4. Push the bump commit and tag — the tag push triggers the CI release workflow,
   which publishes to crates.io and creates a GitHub release.

If you prefer to run the steps manually:

```
cog bump --auto          # bump version, write CHANGELOG.md, commit, tag
git push origin main --follow-tags   # trigger the release workflow
```

Breaking changes (anything that changes the public API) require a
`BREAKING CHANGE:` footer in the commit body so that `cog bump` can correctly
determine the next semver version.
