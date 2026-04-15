---
title: "extractLang()"
description: "Extrae el lenguaje de programación a partir del className de un elemento code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Firma

```ts
function extractLang(code: HastElement): string | null
```

## Parámetros

### code

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identificador del tipo de nodo |
| `tagName` | `string` | — | El nombre de la etiqueta HTML (normalmente `"code"`) |
| `properties` | `Record<string, unknown>` | — | Propiedades del elemento, incluyendo `className` |
| `children` | `HastNode[]` | — | Nodos hijos del elemento |

## Retorna

`string | null` — El identificador de lenguaje extraído de la primera clase `language-*`, o `null` si no se encuentra ninguna clase de lenguaje.

## Uso

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

## Ejemplos

### Extraer el lenguaje de un elemento code

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

### Cuando no existe ninguna clase de lenguaje

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

### Cuando className no es un array

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

### Uso con findCodeChild

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
