# @unifast/node

Node.js binding for [unifast](https://kenzo-pj.github.io/unifast/) — a high-performance Markdown/MDX compiler with a Rust core.

One call compiles Markdown to HTML with all features applied. No JS plugin chain overhead.

## Install

```sh
npm install @unifast/node
```

## Quick Start

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
console.log(result.html); // Rendered HTML with GFM + highlighting
```

## API

### `compile(input, options?)`

Compile a Markdown or MDX string.

- **`input`** (`string`) — Markdown/MDX source
- **`options`** (`CompileOptions`) — Compiler options and plugins

Returns a `CompileResult`:

| Property      | Type                               | Description                                    |
| ------------- | ---------------------------------- | ---------------------------------------------- |
| `output`      | `string \| object`                 | Compiled output (HTML, HAST, MDAST, or MDX-JS) |
| `frontmatter` | `Record<string, unknown>`          | Extracted frontmatter metadata                 |
| `diagnostics` | `Diagnostic[]`                     | Compilation warnings and errors                |
| `stats`       | `{ parseMs, transformMs, emitMs }` | Performance timing                             |
| `toc`         | `TocEntry[]`                       | Table of contents entries                      |
| `readingTime` | `{ minutes, words }`               | Estimated reading time                         |
| `excerpt`     | `string`                           | Content before `<!-- more -->` marker          |

### Built-in Plugins

All plugins are included — no separate install needed. All implemented natively in Rust.

`gfm`, `frontmatter`, `sanitize`, `syntect`, `treeSitter`, `toc`, `externalLinks`, `autolinkHeadings`, `smartypants`, `wikiLink`, `codeImport`, `emoji`, `breaks`, `math`, `githubAlert`, `sectionize`, `directive`, `definitionList`, `rubyAnnotation`, `cjk`, `codeMeta`, `figure`, `customHeadingId`, `readingTime`, `excerpt`, `abbr`, `commentRemoval`, `imgLazyLoading`, `accessibleEmoji`, `addClasses`, `minify`

See the [full plugin documentation](https://github.com/kenzo-pj/unifast#built-in-plugins).

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
