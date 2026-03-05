---
title: "frontmatter()"
description: "Create a frontmatter plugin that enables parsing of YAML, TOML, or JSON metadata blocks at the top of Markdown/MDX documents."
---

```ts
import { frontmatter } from "@unifast/node";
```

### Signature

```ts
function frontmatter(options?: FrontmatterPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `FrontmatterPluginOptions` | Configuration for frontmatter formats |

#### `FrontmatterPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `yaml` | `boolean` | `true` | Enable YAML frontmatter (`---` delimiters) |
| `toml` | `boolean` | `false` | Enable TOML frontmatter (`+++` delimiters) |
| `json` | `boolean` | `false` | Enable JSON frontmatter (opening `{`) |

### Returns

`UnifastPlugin`

## Usage

### Basic usage (YAML)

```ts
import { compile, frontmatter } from "@unifast/node";

const md = `---
title: My Document
author: John Doe
tags:
  - markdown
  - unifast
---

# Content here
`;

const result = compile(md, {
  plugins: [frontmatter()],
});

console.log(result.frontmatter);
// { title: "My Document", author: "John Doe", tags: ["markdown", "unifast"] }
```

### Enable TOML frontmatter

```ts
import { compile, frontmatter } from "@unifast/node";

const md = `+++
title = "My Document"
date = 2025-01-01
+++

# Content
`;

const result = compile(md, {
  plugins: [frontmatter({ toml: true })],
});
```

### Enable JSON frontmatter

```ts
import { compile, frontmatter } from "@unifast/node";

const md = `{
  "title": "My Document",
  "draft": false
}

# Content
`;

const result = compile(md, {
  plugins: [frontmatter({ json: true })],
});
```

### Enable all formats

```ts
const result = compile(md, {
  plugins: [frontmatter({ yaml: true, toml: true, json: true })],
});
```

### Disable YAML (only use TOML)

```ts
const result = compile(md, {
  plugins: [frontmatter({ yaml: false, toml: true })],
});
```

### Combined with other plugins

```ts
import { compile, frontmatter, gfm, toc } from "@unifast/node";

const result = compile(md, {
  plugins: [frontmatter(), gfm(), toc()],
});

console.log(result.frontmatter);
console.log(result.toc);
```
