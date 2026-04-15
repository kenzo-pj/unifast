---
title: "extractLang()"
description: "Bir code elemanının className değerinden programlama dilini çıkarır."
---

```ts
import { extractLang } from "@unifast/core";
```

## İmza

```ts
function extractLang(code: HastElement): string | null
```

## Parametreler

### code

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `type` | `"element"` | — | Düğüm türü tanımlayıcısı |
| `tagName` | `string` | — | HTML etiket adı (genellikle `"code"`) |
| `properties` | `Record<string, unknown>` | — | `className` dahil eleman özellikleri |
| `children` | `HastNode[]` | — | Elemanın alt düğümleri |

## Dönüş Değeri

`string | null` — İlk `language-*` sınıfından çıkarılan dil tanımlayıcısı veya dil sınıfı bulunamazsa `null`.

## Kullanım

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

## Örnekler

### Bir code elemanından dil çıkarma

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

### Dil sınıfı yoksa

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

### className bir dizi değilse

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

### findCodeChild ile kullanım

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
