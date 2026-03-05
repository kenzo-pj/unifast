---
title: "gfm()"
description: "Create a GFM plugin that enables GitHub Flavored Markdown extensions."
---

```ts
import { gfm } from "@unifast/node";
```

### Signature

```ts
function gfm(options?: GfmPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `GfmPluginOptions` | Configuration for individual GFM features |

#### `GfmPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `tables` | `boolean` | `true` | Enable pipe tables with `\|` syntax |
| `taskList` | `boolean` | `true` | Enable `- [x]` / `- [ ]` task list checkboxes |
| `strikethrough` | `boolean` | `true` | Enable `~~deleted~~` strikethrough syntax |
| `footnotes` | `boolean` | `true` | Enable `[^1]` footnote references and definitions |
| `autolink` | `boolean` | `true` | Automatically link bare URLs |

### Returns

`UnifastPlugin`

## Usage

### Enable all GFM features

```ts
import { compile, gfm } from "@unifast/node";

const md = `
| Feature | Status |
|---------|--------|
| Tables  | Done   |

- [x] Task 1
- [ ] Task 2

This is ~~deleted~~ text.

Check out https://example.com for more.

Here is a footnote[^1].

[^1]: Footnote content.
`;

const result = compile(md, { plugins: [gfm()] });
```

### Tables only

```ts
import { compile, gfm } from "@unifast/node";

const result = compile(md, {
  plugins: [
    gfm({
      tables: true,
      taskList: false,
      strikethrough: false,
      footnotes: false,
      autolink: false,
    }),
  ],
});
```

### Disable specific features

```ts
import { compile, gfm } from "@unifast/node";

// Everything except footnotes
const result = compile(md, {
  plugins: [gfm({ footnotes: false })],
});
```

### Combined with other plugins

```ts
import { compile, gfm, frontmatter, sanitize } from "@unifast/node";

const result = compile(md, {
  plugins: [gfm(), frontmatter(), sanitize()],
});
```
