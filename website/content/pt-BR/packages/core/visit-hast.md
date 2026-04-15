---
title: "visitHast()"
description: "Percorre e transforma uma árvore HAST usando uma função visitor."
---

```ts
import { visitHast } from "@unifast/core";
```

## Assinatura

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Parâmetros

### node

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `type` | `string` | — | O tipo do nó (`"root"`, `"element"`, `"text"`, etc.) |
| `children` | `HastNode[]` | — | Nós filhos (para os tipos `"root"` e `"element"`) |

### visitor

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | Uma função chamada para cada nó; retorne um nó para substituir o original ou `void` para mantê-lo |

## Retorna

`HastNode` — Uma nova árvore com qualquer transformação aplicada pela função visitor.

## Uso

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// Tree with text "HELLO WORLD"
```

## Exemplos

### Adicionar uma classe a todos os parágrafos

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// Both <p> elements now have className: ["prose"]
```

### Remover todas as imagens

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// The <img> element has been removed from the tree
```

### Coletar todos os links

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### Envolver code blocks em um container

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> is now wrapped inside <div class="code-block">
```

## Comportamento

- **Imutável:** Retorna uma nova árvore; a árvore original não é modificada
- **Travessia top-down:** O visitor é chamado no pai antes de seus filhos serem visitados
- **Substituição:** Se o visitor retornar um nó, ele substitui o nó atual antes que os filhos sejam percorridos
- **No-op:** Se o visitor retornar `void` (ou `undefined`), o nó original é mantido
- **Recursivo:** Os filhos de nós `"root"` e `"element"` são visitados recursivamente
