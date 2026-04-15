---
title: "findCodeChild()"
description: "Bir üst elemanın içindeki alt <code> elemanını bulur."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## İmza

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parametreler

### element

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `type` | `"element"` | — | Düğüm türü tanımlayıcısı |
| `tagName` | `string` | — | HTML etiket adı (genellikle `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Eleman özellikleri |
| `children` | `HastNode[]` | — | Arama yapılacak alt düğümler |

## Dönüş Değeri

`HastElement | undefined` — `"code"` `tagName`'ine sahip ilk alt eleman veya böyle bir alt eleman yoksa `undefined`.

## Kullanım

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-js"] },
      children: [{ type: "text", value: "const x = 1;" }],
    },
  ],
};

const code = findCodeChild(pre);

console.log(code?.tagName);
// code
```

## Örnekler

### pre elemanı içinde code bulma

```ts
import { findCodeChild, extractLang, extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-rust"] },
      children: [{ type: "text", value: 'fn main() { println!("hello"); }' }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // rust
  console.log(extractText(code));
  // fn main() { println!("hello"); }
}
```

### Alt code elemanı yoksa

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    { type: "text", value: "plain preformatted text" },
  ],
};

const code = findCodeChild(pre);

console.log(code);
// undefined
```

### Sözdizimi vurgulama için visitHast ile kullanım

```ts
import { visitHast, findCodeChild, extractLang } from "@unifast/core";
import type { HastNode, HastElement } from "@unifast/core";

const tree: HastNode = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    const code = findCodeChild(node);
    if (code) {
      const lang = extractLang(code);
      console.log(`Found code block with language: ${lang}`);
      // Found code block with language: js
    }
  }
});
```
