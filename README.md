# unifast

[![Rust CI](https://github.com/kenzo-pj/unifast/actions/workflows/rust.yml/badge.svg)](https://github.com/kenzo-pj/unifast/actions/workflows/rust.yml)
[![TypeScript CI](https://github.com/kenzo-pj/unifast/actions/workflows/typescript.yml/badge.svg)](https://github.com/kenzo-pj/unifast/actions/workflows/typescript.yml)
[![Release](https://github.com/kenzo-pj/unifast/actions/workflows/release.yml/badge.svg)](https://github.com/kenzo-pj/unifast/actions/workflows/release.yml)
[![Codecov](https://codecov.io/gh/kenzo-pj/unifast/graph/badge.svg)](https://codecov.io/gh/kenzo-pj/unifast)
[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json&label=CodSpeed)](https://codspeed.io/kenzo-pj/unifast?utm_source=badge)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

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

## Features

| Feature | Description |
|---------|-------------|
| **Rust Core** | Parser, AST, transforms, and emit — all in native Rust |
| **CommonMark** | Full CommonMark spec compliance |
| **MDX** | JSX expressions and imports in Markdown |
| **Diagnostics** | Precise error spans with line/column mapping |
| **Multiple Outputs** | HTML, HAST (AST JSON), MDX-JS |
| **WASM Support** | Run in the browser via wasm-bindgen |

## Packages

### Core

| Package | Version | Description |
|---------|---------|-------------|
| [`@unifast/node`](packages/node) | [![npm](https://img.shields.io/npm/v/@unifast/node)](https://www.npmjs.com/package/@unifast/node) | Node.js binding, compiler, and built-in plugins |
| [`@unifast/core`](packages/core) | [![npm](https://img.shields.io/npm/v/@unifast/core)](https://www.npmjs.com/package/@unifast/core) | Shared TypeScript types and HAST utilities (dependency of `@unifast/node`) |

### Built-in Plugins

The following plugins are **included in `@unifast/node`** — no separate install needed.
All are implemented natively in Rust for maximum performance.

| Plugin | Description | Replaces (remark / rehype) |
|--------|-------------|---------------------------|
| `gfm` | Tables, task lists, strikethrough, footnotes, autolinks | [remark-gfm](https://github.com/remarkjs/remark-gfm) |
| `frontmatter` | YAML / TOML / JSON metadata extraction | [remark-frontmatter](https://github.com/remarkjs/remark-frontmatter) + [gray-matter](https://github.com/jonschlinkert/gray-matter) |
| `sanitize` | Schema-based HTML allowlist | [rehype-sanitize](https://github.com/rehypejs/rehype-sanitize) |
| `syntect` | Syntax highlighting (Rust-native, 500+ languages) | [rehype-highlight](https://github.com/rehypejs/rehype-highlight) / [rehype-prism](https://github.com/mapbox/rehype-prism) |
| `treeSitter` | Syntax highlighting (Tree-sitter engine) | [rehype-highlight](https://github.com/rehypejs/rehype-highlight) |
| `toc` | Auto-extracted heading tree | [remark-toc](https://github.com/remarkjs/remark-toc) |
| `externalLinks` | Add `rel` / `target` to external links | [rehype-external-links](https://github.com/rehypejs/rehype-external-links) |
| `autolinkHeadings` | Anchor links on headings (prepend / append / wrap) | [rehype-autolink-headings](https://github.com/rehypejs/rehype-autolink-headings) |
| `smartypants` | Curly quotes, em/en dashes, ellipsis | [remark-smartypants](https://github.com/silvenon/remark-smartypants) |
| `wikiLink` | `[[Page]]` / `[[Page\|Label]]` wiki-style links | [remark-wiki-link](https://github.com/landakram/remark-wiki-link) |
| `codeImport` | Import code from external files via `file=` meta | [remark-code-import](https://github.com/kevin940726/remark-code-import) |
| `emoji` | `:shortcode:` → Unicode emoji | [remark-emoji](https://github.com/rhysd/remark-emoji) |
| `breaks` | Newlines → `<br>` (like GitHub) | [remark-breaks](https://github.com/remarkjs/remark-breaks) |
| `math` | `$inline$` and `$$block$$` math notation | [remark-math](https://github.com/remarkjs/remark-math) |
| `githubAlert` | `> [!NOTE]` / `[!WARNING]` callout blocks | [remark-github-blockquote-alert](https://github.com/jaywcjlove/remark-github-blockquote-alert) |
| `sectionize` | Wrap headings + content in `<section>` | [remark-sectionize](https://github.com/jake-low/remark-sectionize) |
| `directive` | `:::name` container directives | [remark-directive](https://github.com/remarkjs/remark-directive) |
| `definitionList` | `Term` / `: Definition` → `<dl>` | [remark-definition-list](https://github.com/wataru-chocola/remark-definition-list) |
| `rubyAnnotation` | `{漢字\|かんじ}` → `<ruby>` | — |
| `cjk` | Remove unnecessary line breaks between CJK chars | [remark-join-cjk-lines](https://github.com/purefun/remark-join-cjk-lines) |
| `codeMeta` | Parse code block meta (title, line highlights, diff) | [rehype-meta-content](https://github.com/rehypejs/rehype-meta-content) |
| `figure` | Wrap images with captions in `<figure>` | [rehype-figure](https://github.com/josestg/rehype-figure) |
| `customHeadingId` | `# Heading {#custom-id}` syntax | [remark-heading-id](https://github.com/imcuttle/remark-heading-id) |
| `readingTime` | Estimated reading time (Latin + CJK aware) | [remark-reading-time](https://github.com/ngryman/reading-time) |
| `excerpt` | Extract summary before `<!-- more -->` marker | [remark-excerpt](https://github.com/manovotny/remark-excerpt) |
| `abbr` | `*[TERM]: Definition` → `<abbr>` | [remark-abbr](https://github.com/zestedesavoir/zmarkdown/tree/master/packages/remark-abbr) |
| `commentRemoval` | Strip HTML comments from output | [remark-remove-comments](https://github.com/alvinometric/remark-remove-comments) |
| `imgLazyLoading` | Add `loading="lazy"` to images | [rehype-img-size](https://github.com/ksoichiro/rehype-img-size) |
| `accessibleEmoji` | Wrap emoji with `aria-label` for a11y | [rehype-accessible-emojis](https://github.com/GaiAma/Coding4GaiAma/tree/master/packages/rehype-accessible-emojis) |
| `addClasses` | Add CSS classes via selectors | [rehype-add-classes](https://github.com/martypdx/rehype-add-classes) |
| `minify` | Minify HTML output | [rehype-preset-minify](https://github.com/rehypejs/rehype-minify) |

### Additional Packages

The following packages have external npm dependencies and require separate installation:

| Package | Version | Description |
|---------|---------|-------------|
| [`@unifast/react`](packages/react) | [![npm](https://img.shields.io/npm/v/@unifast/react)](https://www.npmjs.com/package/@unifast/react) | HAST to React element conversion |
| [`@unifast/shiki`](packages/shiki) | [![npm](https://img.shields.io/npm/v/@unifast/shiki)](https://www.npmjs.com/package/@unifast/shiki) | Shiki syntax highlighting |
| [`@unifast/highlight`](packages/highlight) | [![npm](https://img.shields.io/npm/v/@unifast/highlight)](https://www.npmjs.com/package/@unifast/highlight) | lowlight syntax highlighting |
| [`@unifast/vite`](packages/vite) | [![npm](https://img.shields.io/npm/v/@unifast/vite)](https://www.npmjs.com/package/@unifast/vite) | Vite integration with HMR |

### CLI & Bindings (Rust)

| Crate | Version | Description |
|-------|---------|-------------|
| [`unifast-core`](crates/unifast-core) | [![crates.io](https://img.shields.io/crates/v/unifast-core)](https://crates.io/crates/unifast-core) | Parser, AST, transforms, emit |
| [`unifast-bindings-node`](crates/unifast-bindings-node) | [![crates.io](https://img.shields.io/crates/v/unifast-bindings-node)](https://crates.io/crates/unifast-bindings-node) | napi-rs Node.js binding |
| [`unifast-bindings-wasm`](crates/unifast-bindings-wasm) | [![crates.io](https://img.shields.io/crates/v/unifast-bindings-wasm)](https://crates.io/crates/unifast-bindings-wasm) | wasm-bindgen WASM binding |
| [`unifast-cli`](crates/unifast-cli) | [![crates.io](https://img.shields.io/crates/v/unifast-cli)](https://crates.io/crates/unifast-cli) | Command-line interface |

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
