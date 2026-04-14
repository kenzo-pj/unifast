---
title: "extractText()"
description: "Bir HAST düğümünden tüm metin içeriğini özyinelemeli olarak çıkarır."
---

```ts
import { extractText } from "@unifast/core";
```

## İmza

```ts
function extractText(node: HastNode): string
```

## Parametreler

### node

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `type` | `string` | — | Düğüm türü (`"root"`, `"element"`, `"text"`, vb.) |
| `children` | `HastNode[]` | — | Alt düğümler (`"root"` ve `"element"` türleri için) |
| `value` | `string` | — | Metin içeriği (`"text"` türü için) |

## Dönüş Değeri

`string` — Düğüm ve alt düğümlerinden birleştirilmiş tüm metin içeriği.

## Kullanım

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const element: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "Hello " },
    {
      type: "element",
      tagName: "strong",
      properties: {},
      children: [{ type: "text", value: "world" }],
    },
  ],
};

const text = extractText(element);

console.log(text);
// Hello world
```

## Örnekler

### Basit bir elemandan çıkarma

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h1",
  properties: { id: "title" },
  children: [{ type: "text", value: "Getting Started" }],
};

console.log(extractText(heading));
// Getting Started
```

### İç içe elemanlardan çıkarma

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const paragraph: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "This is " },
    {
      type: "element",
      tagName: "em",
      properties: {},
      children: [
        { type: "text", value: "deeply " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "nested" }],
        },
      ],
    },
    { type: "text", value: " content." },
  ],
};

console.log(extractText(paragraph));
// This is deeply nested content.
```

### Boş eleman

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const emptyDiv: HastElement = {
  type: "element",
  tagName: "div",
  properties: {},
  children: [],
};

console.log(extractText(emptyDiv));
// (boş string)
```

### Başlık slug'larını üretme

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h2",
  properties: {},
  children: [
    { type: "text", value: "API " },
    {
      type: "element",
      tagName: "code",
      properties: {},
      children: [{ type: "text", value: "Reference" }],
    },
  ],
};

const slug = extractText(heading).toLowerCase().replace(/\s+/g, "-");

console.log(slug);
// api-reference
```
