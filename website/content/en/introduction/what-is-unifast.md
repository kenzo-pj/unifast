---
title: "What is unifast?"
description: "unifast is a high-performance Markdown and MDX compiler with a Rust core. Built-in passes for GFM, sanitization, highlighting, and TOC."
---

unifast is a high-performance Markdown and MDX compiler with a Rust core. It covers the mainstream use-cases of remark/rehype by implementing features directly as built-in passes - not through JS plugin compatibility.

### Why unifast?

Traditional Markdown toolchains like unified/remark/rehype are powerful but come with trade-offs:

- **Performance overhead** - Multiple JS AST transformations add up, especially at scale.
- **Plugin coordination** - Ordering, compatibility, and duplication across dozens of plugins.
- **No built-in features** - Even basic tasks like GFM or sanitization require separate packages.

unifast takes a different approach:

- **Rust core** - Parsing, transformation, and emission all happen in native code.
- **Built-in passes** - Common features (GFM, sanitization, highlighting, TOC) are built-in, not bolted on.
- **Single compilation** - One call compiles Markdown to HTML with all features applied.

### Key Features

| Feature | Description |
|---------|-------------|
| **CommonMark + GFM** | Tables, task lists, strikethrough, autolinks, footnotes |
| **Frontmatter** | YAML, TOML, and JSON metadata extraction |
| **MDX** | JSX expressions and imports in Markdown |
| **Diagnostics** | Precise error spans with line/column mapping |

### Built-in Passes

Common remark/rehype plugins are reimplemented as native Rust passes. No npm install, no ordering headaches.

| Pass | Description |
|------|-------------|
| **Sanitization** | Schema-based HTML allowlist with safe defaults |
| **Syntax Highlighting** | Pluggable engines (syntect, Shiki) |
| **Table of Contents** | Auto-extracted heading tree |

### Platform Support

unifast runs on multiple platforms from a single Rust core:

- **`@unifast/node`** - Node.js binding via N-API (napi-rs). Primary target.
- **`@unifast/core`** - TypeScript type definitions shared across all packages.
- **`unifast` (CLI)** - Command-line interface for scripts and CI.
- **WASM** - Browser and edge runtime support (secondary target).

### Non-goals

unifast is **not** a drop-in replacement for unified. It does not:

- Execute existing remark/rehype JS plugins inside the core.
- Provide API compatibility with the unified ecosystem.
- Depend on Node's module resolution in the core compilation path.

Instead, it targets **use-case completeness** - covering what most projects need without the complexity of assembling a plugin pipeline.
