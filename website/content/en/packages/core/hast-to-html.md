---
title: "hastToHtml()"
description: "Serialize a HAST root node into an HTML string."
---

```ts
import { hastToHtml } from "@unifast/core";
```

### Signature

```ts
function hastToHtml(hast: HastRoot): string
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `hast` | `HastRoot` | A HAST root node to serialize |

### Returns

`string` - The serialized HTML string.

## Usage

### Basic serialization

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello" },
      children: [{ type: "text", value: "Hello" }],
    },
  ],
};

const html = hastToHtml(hast);
// <h1 id="hello">Hello</h1>
```

### With nested elements

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
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "em",
          properties: {},
          children: [{ type: "text", value: "italic" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);
// <p>This is <strong>bold</strong> and <em>italic</em></p>
```

### With className arrays

```ts
const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "div",
      properties: { className: ["container", "main"] },
      children: [{ type: "text", value: "Content" }],
    },
  ],
};

const html = hastToHtml(hast);
// <div class="container main">Content</div>
```

### Void elements

Void elements (`<br>`, `<img>`, `<hr>`, etc.) are self-closed automatically:

```ts
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

const html = hastToHtml(hast);
// <img alt="A photo" src="photo.jpg" />
```

### With compile() output

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);
const html = hastToHtml(hast);
// <p><strong>bold text</strong></p>
```

### Raw HTML passthrough

```ts
const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

const html = hastToHtml(hast);
// <div class="custom">Raw HTML</div>
```

## Behavior

- **HTML escaping:** Text content is escaped (`&`, `<`, `>`, `"`)
- **Void elements:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` are self-closed
- **Attributes:** Sorted alphabetically; `className` arrays are joined with spaces and rendered as `class`; boolean `true` renders as bare attribute; `false`/`null`/`undefined` are omitted
- **Comments:** Rendered as `<!--value-->`
- **Doctype:** Rendered as `<!DOCTYPE html>`
- **Raw nodes:** Output as-is without escaping
