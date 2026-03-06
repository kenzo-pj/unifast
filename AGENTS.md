# unifast

High-performance Markdown/MDX compiler with Oxc-style architecture.

## Architecture

- **Rust core** (`crates/unifast-core`): Parser, AST, transforms, emit
- **Node.js binding** (`crates/unifast-bindings-node`): napi-rs based
- **WASM binding** (`crates/unifast-bindings-wasm`): wasm-bindgen based
- **CLI** (`crates/unifast-cli`): clap-based command-line tool
- **TypeScript packages** (`packages/*`): Type definitions and convenience wrappers

## Pipeline

```
Input → Parse → MdAst → Built-in Passes → Lower to HAst → HAst Passes → Emit
```

## Commands

```bash
# Run all tests
cargo nextest run --all --all-features

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Run benchmarks
cargo run --release --bin unifast-bench-compile 2>/dev/null || cargo bench -p unifast-core

# Build CLI
cargo build -p unifast-cli

# Check Node binding
cargo check -p unifast-bindings-node

# Check WASM binding (needs target)
cargo check -p unifast-bindings-wasm --target wasm32-unknown-unknown

# Run tests with coverage
just coverage            # All (Rust + TS)
just coverage-rust       # Rust only (lcov → coverage/rust-lcov.info)
just coverage-ts         # TypeScript only (lcov → packages/*/coverage/)
just coverage-rust-html  # Rust HTML report
```

## Conventions

- Code is self-documenting. Do not write unnecessary comments. If the code needs a comment to be understood, rewrite the code to be clearer instead.
- Rust edition 2024 (`gen` is reserved — use `next_id()` not `next()`)
- Span-first source mapping: every AST node carries `Span { start, end }`
- `SmallMap<K,V>` (BTreeMap wrapper) for stable attribute ordering
- `NodeId(u32)` for unique node identification via `NodeIdGen`
- Arena allocation with `bumpalo` for AST builders
- Tests inline in source files (`#[cfg(test)] mod tests`)
- Snapshot testing with `insta` for complex output comparison

## Key Types

- `MdNode` — 31-variant enum for Markdown AST
- `HNode` — 6-variant enum for HTML AST (Root, Element, Text, Comment, Doctype, Raw)
- `JsNode` — 6-variant enum for JS AST (MDX support)
- `CompileOptions` — All compilation settings
- `CompileResult` — Output + frontmatter + diagnostics + stats

## Crate Structure

```
crates/unifast-core/src/
├── api/          # CompileOptions, CompileResult, compile()
├── ast/          # MdAst, HAst, JsAst definitions
│   ├── common.rs # Span, NodeId, Position
│   ├── mdast/    # Markdown AST (nodes, visitor, builder)
│   ├── hast/     # HTML AST (nodes, visitor, builder)
│   └── jsast/    # JS AST (nodes)
├── parse/        # Markdown, GFM, frontmatter, MDX parsers
├── transform/    # Pass system, registry, built-in passes
├── emit/         # HTML stringify, MDX JS printer, sourcemaps
├── diagnostics/  # DiagLevel, Diagnostic, render
├── cache/        # Memory cache, disk cache stub
└── util/         # LineIndex, Interner, hash, SmallMap
```
