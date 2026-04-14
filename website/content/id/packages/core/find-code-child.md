---
title: "findCodeChild()"
description: "Menemukan elemen <code> anak di dalam sebuah elemen induk."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Signature

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parameter

### element

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identifier tipe node |
| `tagName` | `string` | — | Nama tag HTML (biasanya `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Properti elemen |
| `children` | `HastNode[]` | — | Node anak yang akan dicari |

## Return

`HastElement | undefined` — Elemen anak pertama dengan `tagName` bernilai `"code"`, atau `undefined` jika tidak ada anak seperti itu.

## Penggunaan

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

## Contoh

### Menemukan code di dalam elemen pre

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

### Ketika tidak ada anak code

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

### Penggunaan dengan visitHast untuk syntax highlighting

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
