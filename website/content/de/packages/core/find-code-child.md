---
title: "findCodeChild()"
description: "Findet ein untergeordnetes <code>-Element innerhalb eines übergeordneten Elements."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Signatur

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parameter

### element

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Bezeichner des Knotentyps |
| `tagName` | `string` | — | Der HTML-Tag-Name (typischerweise `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Element-Eigenschaften |
| `children` | `HastNode[]` | — | Zu durchsuchende Kindknoten |

## Rückgabewert

`HastElement | undefined` – Das erste Kindelement mit dem `tagName` `"code"`, oder `undefined`, falls kein solches Kind existiert.

## Verwendung

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

## Beispiele

### Code innerhalb eines pre-Elements finden

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

### Wenn kein Code-Kind existiert

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

### Verwendung mit visitHast zur Syntax-Hervorhebung

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
