# Contributing

## Prerequisites

- Rust (edition 2024)
- Node.js
- pnpm

## Setup

```bash
git clone https://github.com/kenzo-pj/unifast.git
cd unifast
pnpm install
pnpm build
```

## Commands

```bash
# Build all packages
pnpm build

# Run Rust tests
cargo nextest run --all --all-features

# Run JS tests
pnpm test

# Lint
cargo clippy --all-targets --all-features -- -D warnings
pnpm lint

# Format
cargo fmt --all

# Run benchmarks (unifast vs unified)
pnpm bench

# Dev website
pnpm dev:website
```

## Project Structure

```
crates/
  unifast-core/           # Rust core — parser, AST, transforms, emit
  unifast-bindings-node/  # napi-rs Node.js binding
  unifast-bindings-wasm/  # wasm-bindgen WASM binding
  unifast-cli/            # Command-line interface
packages/
  core/                   # TypeScript type definitions
  node/                   # Node.js binding wrapper
  mdx/                    # MDX support
  plugin-*/               # Plugin packages
  benchmark/              # Benchmark suite (unifast vs unified)
website/                  # Documentation site
```

## Conventions

- Rust edition 2024 (`gen` is reserved — use `next_id()` not `next()`)
- Tests inline in source files (`#[cfg(test)] mod tests`)
- Snapshot testing with `insta` for complex output comparison
- Code is self-documenting — avoid unnecessary comments
