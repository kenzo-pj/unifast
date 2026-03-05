---
title: "syntect()"
description: "Create a syntect plugin that enables Rust-native syntax highlighting for fenced code blocks. Powered by Sublime Text syntax definitions with support for 100+ languages."
---

```ts
import { syntect } from "@unifast/node";
```

### Signature

```ts
function syntect(options?: SyntectPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `SyntectPluginOptions` | Highlighting engine configuration |

#### `SyntectPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `engine` | `"none" \| "syntect"` | `"syntect"` | `"syntect"` uses the syntect highlighter; `"none"` disables highlighting |

### Returns

`UnifastPlugin`

## Usage

### Basic usage

```ts
import { compile, syntect } from "@unifast/node";

const md = `
\`\`\`javascript
function greet(name) {
  return \`Hello, \${name}!\`;
}
\`\`\`
`;

const result = compile(md, { plugins: [syntect()] });
// Code blocks will have syntax highlighting with sy-* CSS classes
```

### Disable highlighting (passthrough)

```ts
import { compile, syntect } from "@unifast/node";

const result = compile(md, {
  plugins: [syntect({ engine: "none" })],
});
```

### Combined with GFM

```ts
import { compile, syntect, gfm } from "@unifast/node";

const result = compile(md, {
  plugins: [gfm(), syntect()],
});
```
