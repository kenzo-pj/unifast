---
title: "hastToHtml()"
description: "Serializa un nodo raíz HAST a una cadena HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Firma

```ts
function hastToHtml(hast: HastRoot): string
```

## Parámetros

### hast

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Identificador del tipo de nodo |
| `children` | `HastNode[]` | — | Nodos hijos del árbol |

## Retorna

`string` — La cadena HTML serializada.

## Uso

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

## Ejemplos

### Serialización básica

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

### Elementos vacíos (void)

Los elementos vacíos (`<br>`, `<img>`, `<hr>`, etc.) se autocierran automáticamente:

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

### Con la salida de compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Paso de HTML crudo

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

## Comportamiento

- **Escape HTML:** el contenido de texto se escapa (`&`, `<`, `>`, `"`)
- **Elementos void:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` se autocierran
- **Atributos:** ordenados alfabéticamente; los arrays `className` se unen con espacios y se renderizan como `class`; el booleano `true` se renderiza como un atributo sin valor; `false`/`null`/`undefined` se omiten
- **Comentarios:** se renderizan como `<!--value-->`
- **Doctype:** se renderiza como `<!DOCTYPE html>`
- **Nodos raw:** se emiten tal cual, sin escape
