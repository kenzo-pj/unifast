---
title: "extractText()"
description: "Рекурсивно извлекает всё текстовое содержимое из HAST-узла."
---

```ts
import { extractText } from "@unifast/core";
```

## Сигнатура

```ts
function extractText(node: HastNode): string
```

## Параметры

### node

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `type` | `string` | — | Тип узла (`"root"`, `"element"`, `"text"` и т. д.) |
| `children` | `HastNode[]` | — | Дочерние узлы (для типов `"root"` и `"element"`) |
| `value` | `string` | — | Текстовое содержимое (для типа `"text"`) |

## Возвращаемое значение

`string` — всё текстовое содержимое, объединённое из узла и его потомков.

## Использование

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const element: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "Hello " },
    {
      type: "element",
      tagName: "strong",
      properties: {},
      children: [{ type: "text", value: "world" }],
    },
  ],
};

const text = extractText(element);

console.log(text);
// Hello world
```

## Примеры

### Извлечение из простого элемента

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h1",
  properties: { id: "title" },
  children: [{ type: "text", value: "Getting Started" }],
};

console.log(extractText(heading));
// Getting Started
```

### Извлечение из вложенных элементов

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const paragraph: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "This is " },
    {
      type: "element",
      tagName: "em",
      properties: {},
      children: [
        { type: "text", value: "deeply " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "nested" }],
        },
      ],
    },
    { type: "text", value: " content." },
  ],
};

console.log(extractText(paragraph));
// This is deeply nested content.
```

### Пустой элемент

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const emptyDiv: HastElement = {
  type: "element",
  tagName: "div",
  properties: {},
  children: [],
};

console.log(extractText(emptyDiv));
// (пустая строка)
```

### Генерация слагов для заголовков

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h2",
  properties: {},
  children: [
    { type: "text", value: "API " },
    {
      type: "element",
      tagName: "code",
      properties: {},
      children: [{ type: "text", value: "Reference" }],
    },
  ],
};

const slug = extractText(heading).toLowerCase().replace(/\s+/g, "-");

console.log(slug);
// api-reference
```
