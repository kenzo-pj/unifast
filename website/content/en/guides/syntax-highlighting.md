---
title: "Syntax Highlighting"
description: "Configure code block syntax highlighting with syntect or Shiki"
---

## Syntax Highlighting

unifast supports two syntax highlighting engines: **syntect** (Rust-native, build-time) and **Shiki** (JavaScript, runtime). Choose based on your needs.

### Comparison

| | syntect | Shiki |
|---|---------|-------|
| **Runtime** | Rust (built-in) | JavaScript |
| **Speed** | Fastest - runs inside the compiler | Slower - requires JS runtime |
| **Themes** | TextMate themes | VS Code themes |
| **Languages** | TextMate grammars | VS Code grammars |
| **CSS output** | Class-based (`sy-*`) | Inline styles or class-based |
| **Best for** | SSG, build-time rendering | Fine-grained theme control |

### Using syntect

syntect runs entirely inside the Rust compiler - no additional JavaScript overhead.

```sh
npm install @unifast/plugin-syntect
```

```ts
import { compile } from "@unifast/node";
import { syntect } from "@unifast/plugin-syntect";

const result = compile(
  '```typescript\nconst x: number = 42;\n```',
  { plugins: [syntect()] }
);
```

syntect generates CSS class names prefixed with `sy-`. You need to provide CSS rules for these classes in your stylesheet.

### Using Shiki

Shiki uses VS Code's TextMate grammar engine for accurate highlighting with theme support.

```sh
npm install @unifast/plugin-shiki shiki
```

```ts
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/plugin-shiki";

const shiki = await createShikiPlugin({
  theme: "github-dark",
  langs: ["typescript", "rust", "bash"],
});

const result = compile(
  '```typescript\nconst x: number = 42;\n```',
  { plugins: [shiki] }
);
```

### Line Numbers

Both engines support line numbers. Enable them in the compile options:

```ts
const result = compile(source, {
  plugins: [syntect()],
  lineNumbers: true,
});
```

Each line is wrapped in a `<span>` with a `data-line` attribute for CSS-based styling:

```css
[data-line]::before {
  content: attr(data-line);
  display: inline-block;
  width: 2rem;
  text-align: right;
  margin-right: 1rem;
  color: #6b7280;
}
```

### See Also

- [syntect()](/docs/plugins/plugin-syntect/overview) - syntect plugin API reference
- [createShikiPlugin()](/docs/plugins/plugin-shiki/create-shiki-plugin) - Shiki plugin API reference
