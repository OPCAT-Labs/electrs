# Project Guidelines for Claude

## Overview

This is the electrs indexer — a Rust implementation of an Electrum Server backend,
adapted for the OP_CAT / Layer Bitcoin network. It indexes Bitcoin transactions and
serves Electrum protocol queries.

## Build & Test

```bash
cargo build --all-features   # build
cargo test                   # run tests
cargo fmt                    # format
cargo clippy --all-features  # lint
```

## Code Style

- Rust stable toolchain; no nightly-only features
- Follow existing module structure — do not introduce new modules without precedent
- Use `tracing` for logging (not `println!` or `log`)
- Error handling via `anyhow` in application code, custom error types in library code

## PR Standards

- Keep PRs small and focused on a single concern
- Always include `Fixes #N` or `Closes #N` in PR descriptions when working from an issue
- Run `cargo fmt && cargo clippy && cargo test` before opening a PR

## Commit Standards

- Use conventional commits: `fix:`, `feat:`, `refactor:`, `test:`, `docs:`

## Security

- Never commit private keys or credentials
- Changes to indexing or query logic require careful review
