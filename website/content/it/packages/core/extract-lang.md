---
title: "extractLang()"
description: "Estrae il linguaggio di programmazione dal className di un elemento code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Firma

```ts
function extractLang(code: HastElement): string | null
```

## Parametri

### code

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `type` | `"element"` | — | Identificatore del tipo di nodo |
| `tagName` | `string` | — | Il nome del tag HTML (tipicamente `"code"`) |
| `properties` | `Record<string, unknown>` | — | Proprietà dell'elemento, incluso `className` |
| `children` | `HastNode[]` | — | Nodi figli dell'elemento |

## Valore restituito

`string | null` — L'identificatore del linguaggio estratto dalla prima classe `language-*`, oppure `null` se non viene trovata alcuna classe di linguaggio.

## Utilizzo

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

## Esempi

### Estrarre il linguaggio da un elemento code

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

### Quando non esiste alcuna classe di linguaggio

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

### Quando className non è un array

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

### Utilizzo con findCodeChild

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
