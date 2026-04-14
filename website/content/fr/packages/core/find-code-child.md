---
title: "findCodeChild()"
description: "Recherche un élément enfant <code> à l'intérieur d'un élément parent."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Signature

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Paramètres

### element

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identifiant du type de nœud |
| `tagName` | `string` | — | Le nom de la balise HTML (typiquement `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Propriétés de l'élément |
| `children` | `HastNode[]` | — | Nœuds enfants à parcourir |

## Valeur de retour

`HastElement | undefined` — Le premier élément enfant dont le `tagName` vaut `"code"`, ou `undefined` si aucun enfant de ce type n'existe.

## Utilisation

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

## Exemples

### Trouver un élément de code dans un élément pre

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

### Lorsqu'aucun enfant de code n'existe

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

### Combinaison avec visitHast pour la coloration syntaxique

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
