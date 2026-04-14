---
title: "hastToHtml()"
description: "Sérialise un nœud racine HAST en une chaîne HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Signature

```ts
function hastToHtml(hast: HastRoot): string
```

## Paramètres

### hast

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Identifiant du type de nœud |
| `children` | `HastNode[]` | — | Nœuds enfants de l'arbre |

## Valeur de retour

`string` — La chaîne HTML sérialisée.

## Utilisation

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

## Exemples

### Sérialisation de base

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

### Éléments vides (void elements)

Les éléments vides (`<br>`, `<img>`, `<hr>`, etc.) sont auto-fermés automatiquement :

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

### Avec la sortie de compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Passage direct de HTML brut

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

## Comportement

- **Échappement HTML :** Le contenu textuel est échappé (`&`, `<`, `>`, `"`)
- **Éléments vides :** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` sont auto-fermés
- **Attributs :** Triés par ordre alphabétique ; les tableaux `className` sont joints par des espaces puis rendus sous forme de `class` ; une valeur booléenne `true` est rendue sous forme d'attribut nu ; `false`/`null`/`undefined` sont omis
- **Commentaires :** Rendus sous la forme `<!--value-->`
- **Doctype :** Rendu sous la forme `<!DOCTYPE html>`
- **Nœuds bruts :** Émis tels quels, sans échappement
