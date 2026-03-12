# @unifast/react

Convert [unifast](https://kenzo-pj.github.io/unifast/) compilation output to React elements.

## Install

```sh
npm install @unifast/react @unifast/node react
```

## Quick Start

```tsx
import { compileToReact } from "@unifast/react";
import { createElement, Fragment } from "react";

const { element, frontmatter, toc } = compileToReact("# Hello\n\nWorld", {
  createElement,
  Fragment,
});

// Use `element` directly in JSX
function MyPage() {
  return <article>{element}</article>;
}
```

### Custom Components

```tsx
const { element } = compileToReact(markdown, {
  createElement,
  Fragment,
  components: {
    h1: ({ children }) => <h1 className="title">{children}</h1>,
    a: ({ href, children }) => <Link to={href}>{children}</Link>,
    code: ({ children }) => <CodeBlock>{children}</CodeBlock>,
  },
});
```

## API

### `compileToReact(input, options)`

Compile Markdown to a React element tree.

- **`input`** (`string`) — Markdown/MDX source
- **`options`** (`CompileToReactOptions`) — Extends `CompileOptions` with:
  - `createElement` — React's `createElement` function
  - `Fragment` — React's `Fragment` component
  - `components` — Map of tag names to custom React components

Returns `CompileToReactResult`:

| Property | Type | Description |
|----------|------|-------------|
| `element` | `ReactElement` | Rendered React element tree |
| `frontmatter` | `Record<string, unknown>` | Extracted metadata |
| `diagnostics` | `Diagnostic[]` | Compilation diagnostics |
| `stats` | `object` | Performance timing |
| `toc` | `TocEntry[]` | Table of contents |

### `hastToReact(hast, options)`

Lower-level API: convert a HAST tree directly to React elements.

- **`hast`** (`HastRoot`) — HAST tree from unifast compilation
- **`options`** (`HastToReactOptions`) — `createElement`, `Fragment`, `components`

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
