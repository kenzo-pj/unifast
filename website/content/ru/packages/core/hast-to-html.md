---
title: "hastToHtml()"
description: "Сериализует корневой HAST-узел в строку HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Сигнатура

```ts
function hastToHtml(hast: HastRoot): string
```

## Параметры

### hast

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Идентификатор типа узла |
| `children` | `HastNode[]` | — | Дочерние узлы дерева |

## Возвращаемое значение

`string` — сериализованная строка HTML.

## Использование

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

## Примеры

### Базовая сериализация

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

### Void-элементы

Void-элементы (`<br>`, `<img>`, `<hr>` и др.) автоматически самозакрываются:

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

### С выводом compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### Передача сырого HTML

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

## Поведение

- **Экранирование HTML:** текстовое содержимое экранируется (`&`, `<`, `>`, `"`)
- **Void-элементы:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` самозакрываются
- **Атрибуты:** сортируются по алфавиту; массивы `className` объединяются пробелами и отрисовываются как `class`; булево `true` отрисовывается как атрибут без значения; `false`/`null`/`undefined` опускаются
- **Комментарии:** отрисовываются как `<!--value-->`
- **Doctype:** отрисовывается как `<!DOCTYPE html>`
- **Сырые узлы:** выводятся как есть без экранирования
