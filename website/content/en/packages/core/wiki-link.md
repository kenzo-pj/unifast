---
title: "wikiLink()"
description: "Support wiki-style `[[links]]` with optional aliases."
---

```ts
import { wikiLink } from "@unifast/node";
```

### Signature

```ts
function wikiLink(options?: WikiLinkPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `WikiLinkPluginOptions` | Configuration for wiki link behavior |

#### `WikiLinkPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `hrefTemplate` | `string` | `"/wiki/${slug}"` | Template for generating link URLs. Use `${slug}` as a placeholder for the slugified page name. |

### Returns

`UnifastPlugin`

## Usage

### Basic usage

```ts
import { compile, wikiLink } from "@unifast/node";

const md = `
Check out [[Getting Started]] for an introduction.

See also [[API Reference]].
`;

const result = compile(md, { plugins: [wikiLink()] });
// [[Getting Started]] becomes <a href="/wiki/getting-started">Getting Started</a>
// [[API Reference]] becomes <a href="/wiki/api-reference">API Reference</a>
```

### Aliased links

```ts
import { compile, wikiLink } from "@unifast/node";

const md = `
Read the [[Getting Started|quickstart guide]] to begin.

The [[API Reference|full API docs]] cover all functions.
`;

const result = compile(md, { plugins: [wikiLink()] });
// [[Getting Started|quickstart guide]] becomes <a href="/wiki/getting-started">quickstart guide</a>
```

### Custom hrefTemplate

```ts
import { compile, wikiLink } from "@unifast/node";

const result = compile(md, {
  plugins: [wikiLink({ hrefTemplate: "/docs/${slug}" })],
});
// [[Getting Started]] becomes <a href="/docs/getting-started">Getting Started</a>
```

```ts
import { compile, wikiLink } from "@unifast/node";

const result = compile(md, {
  plugins: [wikiLink({ hrefTemplate: "/pages/${slug}.html" })],
});
// [[Getting Started]] becomes <a href="/pages/getting-started.html">Getting Started</a>
```

### Combined with other plugins

```ts
import { compile, wikiLink, gfm, sanitize } from "@unifast/node";

const result = compile(md, {
  plugins: [wikiLink(), gfm(), sanitize()],
});
```
