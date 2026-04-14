---
title: "extractText()"
description: "Estrae in modo ricorsivo tutto il contenuto testuale da un nodo HAST."
---

```ts
import { extractText } from "@unifast/core";
```

## Firma

```ts
function extractText(node: HastNode): string
```

## Parametri

### node

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `type` | `string` | — | Il tipo di nodo (`"root"`, `"element"`, `"text"`, ecc.) |
| `children` | `HastNode[]` | — | Nodi figli (per i tipi `"root"` ed `"element"`) |
| `value` | `string` | — | Contenuto testuale (per il tipo `"text"`) |

## Valore restituito

`string` — Tutto il contenuto testuale concatenato dal nodo e dai suoi discendenti.

## Utilizzo

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

## Esempi

### Estrazione da un elemento semplice

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

### Estrazione da elementi annidati

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

### Elemento vuoto

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
// (empty string)
```

### Generazione degli slug dei titoli

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
