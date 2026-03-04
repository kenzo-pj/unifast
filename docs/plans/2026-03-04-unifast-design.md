# unifast Implementation Design

## Overview

High-performance Markdown/MDX compiler with Oxc-style Rust core, N-API Node bindings, and WASM support.

## Approach

Milestone-sequential bottom-up (spec.md Milestones 1-10), with stub-first type scaffolding.

## Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Rust edition | 2024 | Latest stable (rustc 1.93.1) |
| Arena allocator | bumpalo | Oxc-style pattern, fast bump allocation |
| String interning | Custom lightweight Interner | Minimal deps, tailored to our needs |
| MDX JS parser | oxc_parser crate | Spec requirement: Oxc-based, no SWC |
| YAML parsing | serde_yaml | De facto standard |
| TOML parsing | toml crate | De facto standard |
| JSON parsing | serde_json | De facto standard |
| Code highlight | syntect | Mature, pure Rust, good language support |
| Node binding | napi-rs (napi crate) | Spec requirement, production-proven |
| WASM binding | wasm-bindgen | Spec requirement |
| TS package manager | pnpm workspace | Spec requirement |
| TS test framework | vitest | Modern, fast, good snapshot support |
| Rust snapshots | insta crate | Industry standard for Rust snapshot testing |

## Milestones

1. Core AST + Span + LineIndex + Diagnostics
2. Markdown parse + GFM parse
3. Frontmatter parse (yaml/toml/json)
4. MdAst normalize + built-in passes (slug/toc/defs)
5. Lowering MdAst -> HAst with rawHtml policies
6. Sanitize + HTML stringify
7. Highlight pass + emit
8. MDX parsing (JSX/ESM/expr) via Oxc
9. MDX JS emission + sourcemaps
10. Node binding + CLI + packaging + CI

## Test Strategy

- Rust unit tests: `#[test]` per module
- Rust snapshot tests: `insta` crate
- Rust integration: golden corpus in `tests/corpus/`
- Node tests: vitest + snapshots
- CI: fmt + clippy + test + snapshot drift check
