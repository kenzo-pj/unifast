---
title: "extractLang()"
description: "Извлекает язык программирования из className элемента code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Сигнатура

```ts
function extractLang(code: HastElement): string | null
```

## Параметры

### code

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Идентификатор типа узла |
| `tagName` | `string` | — | Имя HTML-тега (обычно `"code"`) |
| `properties` | `Record<string, unknown>` | — | Свойства элемента, включая `className` |
| `children` | `HastNode[]` | — | Дочерние узлы элемента |

## Возвращаемое значение

`string | null` — идентификатор языка, извлечённый из первого класса `language-*`, либо `null`, если класс языка не найден.

## Использование

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

## Примеры

### Извлечение языка из элемента code

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

### Когда класс языка отсутствует

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

### Когда className не массив

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

### Использование совместно с findCodeChild

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
