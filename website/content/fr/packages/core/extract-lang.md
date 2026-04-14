---
title: "extractLang()"
description: "Extrait le langage de programmation à partir du className d'un élément de code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Signature

```ts
function extractLang(code: HastElement): string | null
```

## Paramètres

### code

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identifiant du type de nœud |
| `tagName` | `string` | — | Le nom de la balise HTML (typiquement `"code"`) |
| `properties` | `Record<string, unknown>` | — | Propriétés de l'élément, y compris `className` |
| `children` | `HastNode[]` | — | Nœuds enfants de l'élément |

## Valeur de retour

`string | null` — L'identifiant de langage extrait de la première classe `language-*`, ou `null` si aucune classe de langage n'est trouvée.

## Utilisation

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const codeElement: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-typescript"] },
  children: [{ type: "text", value: "const x = 1;" }],
};

const lang = extractLang(codeElement);

console.log(lang);
// typescript
```

## Exemples

### Extraire le langage d'un élément de code

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-js", "highlight"] },
  children: [{ type: "text", value: "console.log('hello');" }],
};

console.log(extractLang(code));
// js
```

### Lorsqu'aucune classe de langage n'est présente

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["highlight"] },
  children: [{ type: "text", value: "plain text" }],
};

console.log(extractLang(code));
// null
```

### Lorsque className n'est pas un tableau

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: {},
  children: [{ type: "text", value: "no classes" }],
};

console.log(extractLang(code));
// null
```

### Combinaison avec findCodeChild

```ts
import { extractLang, findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python"] },
      children: [{ type: "text", value: "print('hello')" }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // python
}
```
