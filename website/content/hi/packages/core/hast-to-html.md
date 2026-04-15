---
title: "hastToHtml()"
description: "एक HAST root node को एक HTML string में serialize करें।"
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

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Node type identifier |
| `children` | `HastNode[]` | — | tree के child nodes |

## Returns

`string` — serialize की गई HTML string।

## उपयोग

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

## उदाहरण

### मूल serialization

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

Void elements (`<br>`, `<img>`, `<hr>`, आदि) अपने आप self-close हो जाते हैं:

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

### compile() output के साथ

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

## व्यवहार

- **HTML escaping:** Text content को escape किया जाता है (`&`, `<`, `>`, `"`)
- **Void elements:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` को self-close किया जाता है
- **Attributes:** alphabetically sorted; `className` arrays को spaces के साथ join करके `class` के रूप में render किया जाता है; boolean `true` एक bare attribute के रूप में render होता है; `false`/`null`/`undefined` omit कर दिए जाते हैं
- **Comments:** `<!--value-->` के रूप में render होते हैं
- **Doctype:** `<!DOCTYPE html>` के रूप में render होता है
- **Raw nodes:** बिना escape किए as-is output होते हैं
