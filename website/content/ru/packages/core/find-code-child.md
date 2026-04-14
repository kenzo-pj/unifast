---
title: "findCodeChild()"
description: "Находит дочерний элемент <code> внутри родительского элемента."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Сигнатура

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Параметры

### element

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Идентификатор типа узла |
| `tagName` | `string` | — | Имя HTML-тега (обычно `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Свойства элемента |
| `children` | `HastNode[]` | — | Дочерние узлы для поиска |

## Возвращаемое значение

`HastElement | undefined` — первый дочерний элемент с `tagName`, равным `"code"`, либо `undefined`, если такого дочернего элемента нет.

## Использование

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

## Примеры

### Поиск code внутри элемента pre

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

### Когда дочерний code отсутствует

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

### Использование совместно с visitHast для подсветки синтаксиса

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
