# @unifast/core

Shared TypeScript type definitions and HAST utilities for [unifast](https://unifast.dev) — a high-performance Markdown/MDX compiler with a Rust core.

## Install

```sh
npm install @unifast/core
```

> **Note:** This package is a dependency of `@unifast/node`. You typically don't need to install it directly unless you're building a custom integration.

## API

### Types

- **`CompileOptions`** — Configuration object for the compiler (input kind, output kind, plugin options)
- **`CompileResult`** — Result of compilation (output, frontmatter, diagnostics, stats, toc, readingTime, excerpt)
- **`UnifastPlugin`** — Plugin interface with `name`, `options`, `hastTransform`, and `mdxJsTransform`
- **`TocEntry`** — Table of contents entry (`depth`, `text`, `slug`)
- **`SanitizeSchema`** — HTML sanitization configuration

### HAST Types

- `HastRoot`, `HastElement`, `HastText`, `HastRaw`, `HastComment`, `HastDoctype`

### Utilities

- **`hastToHtml(hast)`** — Convert a HAST tree to an HTML string
- **`escapeHtml(str)`** — Escape HTML special characters
- **`extractLang(node)`** — Extract the language from a code block node
- **`extractText(node)`** — Extract text content from a HAST node
- **`findCodeChild(node)`** — Find the `<code>` child inside a `<pre>` node
- **`visitHast(tree, visitor)`** — Walk a HAST tree

### Built-in Plugins

All 30+ built-in plugins are exported as factory functions (e.g., `gfm()`, `frontmatter()`, `syntect()`, `toc()`, etc.). See the [main README](https://github.com/kenzo-pj/unifast#built-in-plugins) for the full list.

### Error Classes

- `UnifastError`, `ParseError`, `CompileError`

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
