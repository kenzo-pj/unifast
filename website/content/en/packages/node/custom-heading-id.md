---
title: "customHeadingId()"
description: "Set custom IDs on headings using the {#custom-id} syntax."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Signature

```ts
function customHeadingId(): UnifastPlugin
```

## Parameters

None.

## Usage

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Examples

### Custom ID

Use the `{#custom-id}` syntax at the end of a heading to assign a specific `id` attribute. The brace block is removed from the rendered text:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Heading without custom ID

Headings without the `{#...}` syntax are left unchanged. They use the default slug generated from the heading text:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Classes and arbitrary attributes

The brace syntax also supports `.class` and `key=value` notation:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
