---
title: "findCodeChild()"
description: "Encuentra un elemento hijo <code> dentro de un elemento padre."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Firma

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parámetros

### element

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identificador del tipo de nodo |
| `tagName` | `string` | — | El nombre de la etiqueta HTML (normalmente `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Propiedades del elemento |
| `children` | `HastNode[]` | — | Nodos hijos en los que buscar |

## Retorna

`HastElement | undefined` — El primer elemento hijo con `tagName` igual a `"code"`, o `undefined` si no existe dicho hijo.

## Uso

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

## Ejemplos

### Encontrar code dentro de un elemento pre

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

### Cuando no existe un hijo code

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

### Uso con visitHast para resaltado de sintaxis

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
