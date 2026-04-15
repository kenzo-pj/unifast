---
title: "visitHast()"
description: "Durchläuft und transformiert einen HAST-Baum mithilfe einer Visitor-Funktion."
---

```ts
import { visitHast } from "@unifast/core";
```

## Signatur

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Parameter

### node

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `type` | `string` | — | Der Knotentyp (`"root"`, `"element"`, `"text"` usw.) |
| `children` | `HastNode[]` | — | Kindknoten (für die Typen `"root"` und `"element"`) |

### visitor

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | Eine Funktion, die für jeden Knoten aufgerufen wird; geben Sie einen Knoten zurück, um den Originalknoten zu ersetzen, oder `void`, um ihn beizubehalten |

## Rückgabewert

`HastNode` – Ein neuer Baum mit allen durch die Visitor-Funktion angewendeten Transformationen.

## Verwendung

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

## Beispiele

### Eine Klasse zu allen Absätzen hinzufügen

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

### Alle Bilder entfernen

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

### Alle Links sammeln

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

### Code-Blöcke in einen Container einwickeln

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

## Verhalten

- **Unveränderlich:** Gibt einen neuen Baum zurück; der ursprüngliche Baum wird nicht verändert
- **Top-down-Traversierung:** Der Visitor wird auf dem Elternknoten aufgerufen, bevor seine Kinder besucht werden
- **Ersetzung:** Gibt der Visitor einen Knoten zurück, ersetzt dieser den aktuellen Knoten, bevor die Kinder durchlaufen werden
- **No-op:** Gibt der Visitor `void` (oder `undefined`) zurück, bleibt der ursprüngliche Knoten erhalten
- **Rekursiv:** Die Kinder von `"root"`- und `"element"`-Knoten werden rekursiv besucht
