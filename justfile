# unifast justfile
# Usage: just <recipe>

# List available recipes
default:
    @just --list

# --- Build ---

# Build all TypeScript packages
build:
    pnpm build

# Build Rust CLI
build-cli:
    cargo build -p unifast-cli

# Build Rust CLI (release)
build-cli-release:
    cargo build -p unifast-cli --release

# --- Check ---

# Cargo check all targets
check:
    cargo check --all-targets --all-features

# Check Node.js binding
check-node:
    cargo check -p unifast-bindings-node

# Check WASM binding
check-wasm:
    cargo check -p unifast-bindings-wasm --target wasm32-unknown-unknown

# --- Test ---

# Run all tests (Rust + TypeScript)
test: test-rust test-ts

# Run Rust tests
test-rust:
    cargo nextest run --all --all-features

# Run TypeScript tests
test-ts:
    pnpm test

# --- Coverage ---

# Run Rust tests with coverage (requires cargo-llvm-cov)
coverage-rust:
    cargo llvm-cov --all-features --workspace --lcov --output-path coverage/rust-lcov.info

# Run TypeScript tests with coverage
coverage-ts:
    pnpm test:coverage

# Run all tests with coverage
coverage: coverage-rust coverage-ts

# Open Rust coverage HTML report
coverage-rust-html:
    cargo llvm-cov --all-features --workspace --html
    @echo "Report: target/llvm-cov/html/index.html"

# --- Lint ---

# Run all linters
lint:
    pnpm lint

# Fix all auto-fixable lint issues
lint-fix:
    pnpm lint:fix

# Run oxlint (JS/TS)
lint-oxc:
    pnpm lint:oxc

# Run Rust clippy
lint-rust:
    pnpm lint:rust

# Run TypeScript type check
typecheck:
    pnpm typecheck

# Run stylelint on CSS
lint-css:
    pnpm lint:css

# Run markdownlint
lint-md:
    pnpm lint:md

# Run textlint (Japanese)
lint-text:
    pnpm lint:text

# --- Format ---

# Format all code (JS/TS + Rust)
fmt:
    pnpm fmt

# Check formatting without modifying
fmt-check:
    pnpm fmt:check

# Format JS/TS with oxfmt
fmt-oxc:
    pnpm fmt:oxc

# Format Rust with cargo fmt
fmt-rust:
    pnpm fmt:rust

# --- Dev ---

# Start website dev server
dev:
    pnpm dev:website

# Build website
build-website:
    pnpm build:website

# Preview website
preview:
    pnpm preview:website

# --- Bench ---

# Run benchmarks
bench:
    cargo run --release --bin unifast-bench-compile 2>/dev/null || cargo bench -p unifast-core

# --- Release ---

# Release a new version (patch, minor, or major)
release level:
    ./scripts/release.sh {{level}}

# Preview a release without making changes
release-dry-run level:
    ./scripts/release.sh {{level}} --dry-run

# --- Clean ---

# Clean all build artifacts
clean:
    cargo clean
    rm -rf node_modules/.cache
    turbo clean
