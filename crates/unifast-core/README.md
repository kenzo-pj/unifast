# unifast-core

[![Crates.io](https://img.shields.io/crates/v/unifast-core)](https://crates.io/crates/unifast-core)
[![docs.rs](https://docs.rs/unifast-core/badge.svg)](https://docs.rs/unifast-core)

The Rust core of [unifast](https://unifast.dev) — a high-performance Markdown/MDX compiler.

This crate provides the parser, AST types, transformation passes, and emit stages. Everything from parsing Markdown to producing HTML runs natively in Rust.

## Features

- **CommonMark** compliant parser
- **MDX** support (JSX expressions and imports)
- **GFM** extensions (tables, task lists, strikethrough, footnotes, autolinks)
- **30+ built-in passes** (frontmatter, syntax highlighting, TOC, sanitization, etc.)
- **Multiple output formats**: HTML, HAST (JSON), MDAST (JSON), MDX-JS
- **Diagnostics** with precise source spans

## Architecture

```
Input text
  -> Parse (Markdown or MDX)
  -> MdAst (Markdown AST)
  -> Built-in MdAst passes (normalize, slug, TOC, definition resolution)
  -> Lower to HAst (HTML AST)
  -> HAst passes (sanitize, highlight, cleanup)
  -> Emit (HTML / MDX-JS / AST JSON)
```

## Feature Flags

- **`highlight`** (default) — Enables syntax highlighting via Syntect and Tree-sitter (30+ languages)

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
