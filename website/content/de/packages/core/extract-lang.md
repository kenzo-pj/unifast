---
title: "extractLang()"
description: "Extrahiert die Programmiersprache aus der className eines Code-Elements."
---

```ts
import { extractLang } from "@unifast/core";
```

## Signatur

```ts
function extractLang(code: HastElement): string | null
```

## Parameter

### code

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Bezeichner des Knotentyps |
| `tagName` | `string` | — | Der HTML-Tag-Name (typischerweise `"code"`) |
| `properties` | `Record<string, unknown>` | — | Element-Eigenschaften, einschließlich `className` |
| `children` | `HastNode[]` | — | Kindknoten des Elements |

## Rückgabewert

`string | null` – Der Sprachbezeichner, der aus der ersten `language-*`-Klasse extrahiert wurde, oder `null`, falls keine Sprachklasse gefunden wird.

## Verwendung

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

## Beispiele

### Sprache aus einem Code-Element extrahieren

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

### Wenn keine Sprachklasse vorhanden ist

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

### Wenn className kein Array ist

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

### Verwendung mit findCodeChild

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
