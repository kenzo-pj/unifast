---
title: "extractLang()"
description: "एक code element के className से programming language निकालें।"
---

```ts
import { extractLang } from "@unifast/core";
```

## Signature

```ts
function extractLang(code: HastElement): string | null
```

## Parameters

### code

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Node type identifier |
| `tagName` | `string` | — | HTML tag name (आमतौर पर `"code"`) |
| `properties` | `Record<string, unknown>` | — | Element properties, जिसमें `className` शामिल है |
| `children` | `HastNode[]` | — | element के child nodes |

## Returns

`string | null` — पहले `language-*` class से निकाला गया language identifier, या यदि कोई language class नहीं मिलती तो `null`।

## उपयोग

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

## उदाहरण

### एक code element से language निकालें

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

### जब कोई language class नहीं है

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

### जब className एक array नहीं है

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

### findCodeChild के साथ उपयोग

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
