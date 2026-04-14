---
title: "extractLang()"
description: "Extrai a linguagem de programação do className de um elemento de código."
---

```ts
import { extractLang } from "@unifast/core";
```

## Assinatura

```ts
function extractLang(code: HastElement): string | null
```

## Parâmetros

### code

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Identificador do tipo de nó |
| `tagName` | `string` | — | O nome da tag HTML (tipicamente `"code"`) |
| `properties` | `Record<string, unknown>` | — | Propriedades do elemento, incluindo `className` |
| `children` | `HastNode[]` | — | Nós filhos do elemento |

## Retorna

`string | null` — O identificador da linguagem extraído da primeira classe `language-*`, ou `null` se nenhuma classe de linguagem for encontrada.

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

## Exemplos

### Extrair linguagem de um elemento de código

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

### Quando não existe classe de linguagem

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

### Quando className não é um array

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

### Usando com findCodeChild

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
