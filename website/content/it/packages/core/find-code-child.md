---
title: "findCodeChild()"
description: "Trova un elemento figlio <code> all'interno di un elemento padre."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Firma

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parametri

### element

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `type` | `"element"` | — | Identificatore del tipo di nodo |
| `tagName` | `string` | — | Il nome del tag HTML (tipicamente `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Proprietà dell'elemento |
| `children` | `HastNode[]` | — | Nodi figli in cui effettuare la ricerca |

## Valore restituito

`HastElement | undefined` — Il primo elemento figlio con `tagName` pari a `"code"`, oppure `undefined` se non esiste alcun figlio di questo tipo.

## Utilizzo

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

## Esempi

### Trovare il code all'interno di un elemento pre

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

### Quando non esiste un figlio code

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

### Utilizzo con visitHast per l'evidenziazione della sintassi

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
