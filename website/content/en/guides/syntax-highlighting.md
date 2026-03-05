---
title: "Syntax Highlighting"
description: "Choose between syntect (Rust-native, build-time) and Shiki (JavaScript, runtime) syntax highlighting engines in unifast."
---

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

syntect is included in `@unifast/node` — no separate install needed.

```ts
import { compile, syntect } from "@unifast/node";

const result = compile(
  '```typescript\nconst x: number = 42;\n```',
  { plugins: [syntect()] }
);
```

syntect generates CSS class names prefixed with `sy-`. You need to provide CSS rules for these classes in your stylesheet.

### Using Shiki

Shiki uses VS Code's TextMate grammar engine for accurate highlighting with theme support.

```sh
npm install @unifast/shiki shiki
```

```ts
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";

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
import { compile, syntect } from "@unifast/node";

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

- [syntect()](/docs/packages/node/syntect) - syntect API reference
- [createShikiPlugin()](/docs/packages/shiki/create-shiki-plugin) - Shiki API reference
