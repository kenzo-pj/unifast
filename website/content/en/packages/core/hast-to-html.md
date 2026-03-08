---
title: "hastToHtml()"
description: "Serialize a HAST root node into an HTML string."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Signature

```ts
function hastToHtml(hast: HastRoot): string
```

## Parameters

### hast

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Node type identifier |
| `children` | `HastNode[]` | — | Child nodes of the tree |

## Returns

`string` — The serialized HTML string.

## Usage

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## Examples

### Basic serialization

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### Void elements

Void elements (`<br>`, `<img>`, `<hr>`, etc.) are self-closed automatically:

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### With compile() output

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Raw HTML passthrough

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## Behavior

- **HTML escaping:** Text content is escaped (`&`, `<`, `>`, `"`)
- **Void elements:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` are self-closed
- **Attributes:** Sorted alphabetically; `className` arrays are joined with spaces and rendered as `class`; boolean `true` renders as bare attribute; `false`/`null`/`undefined` are omitted
- **Comments:** Rendered as `<!--value-->`
- **Doctype:** Rendered as `<!DOCTYPE html>`
- **Raw nodes:** Output as-is without escaping
