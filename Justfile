# Justfile for serde-structprop
# Install just: https://just.systems

# Default: list available recipes
default:
    @just --list

# Run the full test suite
test:
    cargo test

# Check formatting and lints (mirrors CI)
check:
    cargo fmt --check
    cargo clippy --all-targets -- -W clippy::pedantic

# Auto-format the source
fmt:
    cargo fmt

# Validate conventional commit history
commits:
    cog check

# ---------------------------------------------------------------------------
# Release
# ---------------------------------------------------------------------------

# Bump the version, generate CHANGELOG.md, commit, tag, and push.
#
# Uses `cog bump --auto` to determine the next semver version from the
# conventional commit history, then pushes both the bump commit and the
# new version tag so that the CI release workflow publishes to crates.io.
#
# Prerequisites:
#   - Working tree must be clean (no uncommitted changes).
#   - Must be on the `main` branch and up to date with origin.
#   - `cocogitto` must be installed (`cargo install cocogitto`).
release:
    #!/usr/bin/env bash
    set -euo pipefail

    # Guard: must be on main.
    branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$branch" != "main" ]]; then
        echo "error: releases must be cut from main (currently on '$branch')" >&2
        exit 1
    fi

    # Guard: working tree must be clean.
    if ! git diff --quiet || ! git diff --cached --quiet; then
        echo "error: working tree has uncommitted changes; commit or stash them first" >&2
        exit 1
    fi

    # Guard: local main must not be behind origin.
    git fetch --quiet origin main
    if [[ $(git rev-list --count HEAD..origin/main) -gt 0 ]]; then
        echo "error: local main is behind origin/main; run 'git pull' first" >&2
        exit 1
    fi

    # Run the full test suite before touching anything.
    echo "==> Running tests..."
    cargo test

    # Check formatting and lints.
    echo "==> Checking formatting and lints..."
    cargo fmt --check
    cargo clippy --all-targets -- -W clippy::pedantic

    # Bump version, update Cargo.lock, generate CHANGELOG.md, commit, and tag.
    # cog bump --auto:
    #   - Reads conventional commits since the last tag to pick major/minor/patch
    #   - Updates the version field in Cargo.toml
    #   - Writes CHANGELOG.md
    #   - Creates a signed commit "chore(version): bump to vX.Y.Z"
    #   - Creates an annotated tag vX.Y.Z
    echo "==> Bumping version with cog..."
    cog bump --auto

    # Push the bump commit and the new tag.  The tag push triggers release.yml
    # which runs CI, publishes to crates.io, and creates a GitHub release.
    echo "==> Pushing commit and tag..."
    git push origin main --follow-tags

    echo "==> Done. Monitor the release workflow at:"
    echo "    https://github.com/anthonyoteri/serde-structprop/actions"
