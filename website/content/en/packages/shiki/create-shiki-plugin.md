---
title: "createShikiPlugin()"
description: "Create a Shiki-powered syntax highlighting plugin. An async function that initializes the Shiki highlighter and returns a UnifastPlugin with a HAST transform."
---

```ts
import { createShikiPlugin } from "@unifast/shiki";
```

### Signature

```ts
async function createShikiPlugin(
  options?: ShikiTransformerOptions,
): Promise<UnifastPlugin>
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `ShikiTransformerOptions` | Shiki configuration (themes, languages) |

#### `ShikiTransformerOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `themes` | `BundledTheme[]` | `["github-dark"]` | Shiki themes to load |
| `defaultTheme` | `BundledTheme` | First theme in `themes` | Default theme for rendering |
| `langs` | `BundledLanguage[]` | `[]` | Languages to load. Only loaded languages will be highlighted. |

> `BundledTheme` and `BundledLanguage` are types from the `shiki` package.

### Returns

`Promise<UnifastPlugin>` - The returned plugin:

- Sets `highlight.enabled: false` to disable the built-in Rust highlighter
- Registers a `hastTransform` that applies Shiki highlighting to `<pre>` elements

## Usage

### Basic usage

```ts
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";

const shikiPlugin = await createShikiPlugin({
  themes: ["github-dark"],
  langs: ["typescript", "javascript", "rust", "bash"],
});

const md = `
\`\`\`typescript
const message: string = "Hello, unifast!";
console.log(message);
\`\`\`
`;

const result = compile(md, { plugins: [shikiPlugin] });
```

### Multiple themes

```ts
import { createShikiPlugin } from "@unifast/shiki";

const shikiPlugin = await createShikiPlugin({
  themes: ["github-dark", "github-light", "dracula"],
  defaultTheme: "github-dark",
  langs: ["typescript", "python", "go"],
});
```

### Combined with other plugins

```ts
import { compile, gfm, frontmatter } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";

const shikiPlugin = await createShikiPlugin({
  themes: ["github-dark"],
  langs: ["typescript", "bash"],
});

const result = compile(md, {
  plugins: [gfm(), frontmatter(), shikiPlugin],
});
```

### Reuse across multiple compilations

```ts
import { compile } from "@unifast/node";
import { createShikiPlugin } from "@unifast/shiki";

// Initialize once
const shikiPlugin = await createShikiPlugin({
  themes: ["github-dark"],
  langs: ["typescript", "javascript"],
});

// Reuse for multiple files
const results = files.map((file) =>
  compile(file.content, { plugins: [shikiPlugin] })
);
```
