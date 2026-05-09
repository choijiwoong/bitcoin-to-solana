# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Solana smart contract (program) built with the Anchor framework. Currently implements a simple counter with `initialize` and `increment` instructions. The project is structured as a Cargo workspace.

## Build & Test Commands

```bash
# Build the Solana program (required before running tests)
cargo build-sbf

# Run all tests
cargo test

# Run a single test
cargo test test_initialize

# Lint
cargo clippy

# Format
cargo fmt
```

> Tests use `include_bytes!("../../../target/deploy/test_project.so")`, so **`cargo build-sbf` must be run before `cargo test`** or tests will fail to compile.

## Toolchain

Rust `1.89.0` (stable) with `rustfmt` and `clippy`, pinned in `rust-toolchain.toml`. Anchor `1.0.2`.

## Architecture

The program lives in `programs/test_project/src/`:

- `lib.rs` — The main entry point. Declares the program ID, defines the `#[program]` module with instruction handlers (`initialize`, `increment`), the `#[derive(Accounts)]` context structs, and the `#[account]` data struct `Counter`. **This is where most of the program logic lives.**
- `instructions/` — Scaffold module for splitting instruction handlers into separate files (currently only contains an empty `Initialize` stub; actual logic remains in `lib.rs`).
- `state.rs` — Intended for account data structs (currently empty; `Counter` is in `lib.rs`).
- `constants.rs` — Program-wide constants (currently has the `SEED` anchor constant).
- `error.rs` — Custom `#[error_code]` enum for program errors.

### Testing with LiteSVM

Tests (`programs/test_project/tests/test_initialize.rs`) use [`litesvm`](https://github.com/LiteSVM/litesvm) — a lightweight in-process Solana VM — instead of spinning up a full local validator. The test pattern is:

1. Create a `LiteSVM` instance and load the compiled `.so` binary
2. Airdrop SOL to a payer keypair
3. Build instructions manually using `InstructionData` + `ToAccountMetas` from Anchor
4. Send a `VersionedTransaction` and assert the result

### Anchor.toml

- Cluster: `localnet`
- Wallet: `~/.config/solana/id.json`
- Test script: `cargo test`
- `skip-lint = false` (Anchor lint checks are enabled)

### .gitignore notes

All `*.json` files are ignored except `package.json` and `tsconfig.json` — this is intentional to prevent wallet keypair leaks. Never commit any keypair `.json` files.
