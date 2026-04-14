---
title: "hastToHtml()"
description: "Serializa um nó raiz HAST em uma string HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Assinatura

```ts
function hastToHtml(hast: HastRoot): string
```

## Parâmetros

### hast

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Identificador do tipo de nó |
| `children` | `HastNode[]` | — | Nós filhos da árvore |

## Retorna

`string` — A string HTML serializada.

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

## Exemplos

### Serialização básica

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

### Elementos void

Elementos void (`<br>`, `<img>`, `<hr>`, etc.) são fechados automaticamente:

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

### Com a saída de compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Passthrough de HTML bruto

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

- **Escape de HTML:** O conteúdo textual é escapado (`&`, `<`, `>`, `"`)
- **Elementos void:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` são auto-fechados
- **Atributos:** Ordenados alfabeticamente; arrays `className` são unidos com espaços e renderizados como `class`; `true` booleano é renderizado como atributo isolado; `false`/`null`/`undefined` são omitidos
- **Comentários:** Renderizados como `<!--value-->`
- **Doctype:** Renderizado como `<!DOCTYPE html>`
- **Nós raw:** Saída como está, sem escape
