# unifast

High-performance Markdown and MDX compiler with a Rust core.

unifast covers the mainstream use-cases of remark/rehype by implementing features directly as built-in passes — not through JS plugin compatibility. One call compiles Markdown to HTML with all features applied.

## Quick Start

```sh
npm install @unifast/node
```

```ts
import { compile } from "@unifast/node";

const result = compile("# Hello, unifast!\n\nThis is **Markdown**.");

console.log(result.html);
// <h1>Hello, unifast!</h1>
// <p>This is <strong>Markdown</strong>.</p>
```

### With Plugins

```ts
import { compile, frontmatter, gfm, syntect } from "@unifast/node";

const result = compile(source, {
  plugins: [frontmatter(), gfm(), syntect()],
});

console.log(result.frontmatter); // { title: "My Post", ... }
console.log(result.html);        // Rendered HTML with GFM + highlighting
```

### With Vite

```ts
// vite.config.ts
import { defineConfig } from "vite";
import unifast from "@unifast/vite";
import { frontmatter, gfm } from "@unifast/node";

export default defineConfig({
  plugins: [unifast({ plugins: [frontmatter(), gfm()] })],
});
```

```ts
import content from "./docs/getting-started.md";

content.html;         // Compiled HTML
content.frontmatter;  // Parsed metadata
content.toc;          // Table of contents
```

## Features

| Feature | Description |
|---------|-------------|
| **CommonMark + GFM** | Tables, task lists, strikethrough, autolinks, footnotes |
| **Frontmatter** | YAML, TOML, and JSON metadata extraction |
| **MDX** | JSX expressions and imports in Markdown |
| **Sanitization** | Schema-based HTML allowlist with safe defaults |
| **Syntax Highlighting** | Pluggable engines — syntect (Rust-native) or Shiki |
| **Table of Contents** | Auto-extracted heading tree |
| **Diagnostics** | Precise error spans with line/column mapping |

## Packages

### Core

| Package | Description |
|---------|-------------|
| [`@unifast/node`](packages/node) | Node.js binding via N-API |
| [`@unifast/core`](packages/core) | TypeScript type definitions |

### Plugins

The following plugins are **included in `@unifast/core`** and re-exported from `@unifast/node` — no separate install needed:

`gfm`, `frontmatter`, `sanitize`, `syntect`, `treeSitter`, `toc`, `externalLinks`, `autolinkHeadings`, `smartypants`, `wikiLink`, `codeImport`, `emoji`, `breaks`, `math`, `githubAlert`, `sectionize`, `directive`, `definitionList`, `rubyAnnotation`, `cjk`

The following plugins remain **separate packages** (they have external npm dependencies):

| Package | Description |
|---------|-------------|
| [`@unifast/react`](packages/react) | HAST to React element conversion |
| [`@unifast/shiki`](packages/shiki) | Shiki syntax highlighting |
| [`@unifast/highlight`](packages/highlight) | lowlight syntax highlighting |
| [`@unifast/vite`](packages/vite) | Vite integration with HMR |

### CLI & Bindings (Rust)

| Crate | Description |
|-------|-------------|
| [`unifast-core`](crates/unifast-core) | Parser, AST, transforms, emit |
| [`unifast-bindings-node`](crates/unifast-bindings-node) | napi-rs Node.js binding |
| [`unifast-bindings-wasm`](crates/unifast-bindings-wasm) | wasm-bindgen WASM binding |
| [`unifast-cli`](crates/unifast-cli) | Command-line interface |

## Architecture

```
Input text
  → Parse (Markdown or MDX)
  → MdAst (Markdown AST)
  → Built-in MdAst passes (normalize, slug, TOC, definition resolution)
  → Lower to HAst (HTML AST)
  → HAst passes (sanitize, highlight, cleanup)
  → Emit (HTML / MDX-JS / AST JSON)
```

Plugins don't run arbitrary JavaScript during compilation. They configure built-in passes via options, keeping the hot path entirely in native Rust code.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
