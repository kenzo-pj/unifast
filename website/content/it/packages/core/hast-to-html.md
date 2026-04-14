---
title: "hastToHtml()"
description: "Serializza un nodo radice HAST in una stringa HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Firma

```ts
function hastToHtml(hast: HastRoot): string
```

## Parametri

### hast

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `type` | `"root"` | — | Identificatore del tipo di nodo |
| `children` | `HastNode[]` | — | Nodi figli dell'albero |

## Valore restituito

`string` — La stringa HTML serializzata.

## Utilizzo

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## Esempi

### Serializzazione di base

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### Elementi void

Gli elementi void (`<br>`, `<img>`, `<hr>`, ecc.) vengono auto-chiusi automaticamente:

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### Con l'output di compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Passthrough di HTML raw

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## Comportamento

- **Escape HTML:** Il contenuto testuale viene sottoposto a escape (`&`, `<`, `>`, `"`)
- **Elementi void:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` vengono auto-chiusi
- **Attributi:** Ordinati alfabeticamente; gli array `className` vengono uniti con spazi e resi come `class`; il valore booleano `true` viene reso come attributo nudo; `false`/`null`/`undefined` vengono omessi
- **Commenti:** Resi come `<!--value-->`
- **Doctype:** Reso come `<!DOCTYPE html>`
- **Nodi raw:** Emessi così come sono, senza escape
