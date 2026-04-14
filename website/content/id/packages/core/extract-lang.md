---
title: "extractLang()"
description: "Mengekstrak bahasa pemrograman dari className sebuah elemen code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Signature

```ts
function extractLang(code: HastElement): string | null
```

## Parameter

### code

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identifier tipe node |
| `tagName` | `string` | — | Nama tag HTML (biasanya `"code"`) |
| `properties` | `Record<string, unknown>` | — | Properti elemen, termasuk `className` |
| `children` | `HastNode[]` | — | Node anak dari elemen |

## Return

`string | null` — Identifier bahasa yang diekstrak dari class `language-*` pertama, atau `null` jika tidak ada class bahasa yang ditemukan.

## Penggunaan

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const codeElement: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-typescript"] },
  children: [{ type: "text", value: "const x = 1;" }],
};

const lang = extractLang(codeElement);

console.log(lang);
// typescript
```

## Contoh

### Mengekstrak bahasa dari elemen code

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-js", "highlight"] },
  children: [{ type: "text", value: "console.log('hello');" }],
};

console.log(extractLang(code));
// js
```

### Ketika tidak ada class bahasa

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["highlight"] },
  children: [{ type: "text", value: "plain text" }],
};

console.log(extractLang(code));
// null
```

### Ketika className bukan array

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: {},
  children: [{ type: "text", value: "no classes" }],
};

console.log(extractLang(code));
// null
```

### Penggunaan dengan findCodeChild

```ts
import { extractLang, findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python"] },
      children: [{ type: "text", value: "print('hello')" }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // python
}
```
