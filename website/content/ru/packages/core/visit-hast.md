---
title: "visitHast()"
description: "Обходит и преобразует HAST-дерево с помощью функции-посетителя."
---

```ts
import { visitHast } from "@unifast/core";
```

## Сигнатура

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Параметры

### node

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `type` | `string` | — | Тип узла (`"root"`, `"element"`, `"text"` и т. д.) |
| `children` | `HastNode[]` | — | Дочерние узлы (для типов `"root"` и `"element"`) |

### visitor

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | Функция, вызываемая для каждого узла; вернёт узел — исходный будет заменён, вернёт `void` — останется без изменений |

## Возвращаемое значение

`HastNode` — новое дерево с преобразованиями, применёнными функцией-посетителем.

## Использование

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// Дерево с текстом "HELLO WORLD"
```

## Примеры

### Добавление класса ко всем абзацам

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// Оба элемента <p> теперь имеют className: ["prose"]
```

### Удаление всех изображений

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// Элемент <img> был удалён из дерева
```

### Сбор всех ссылок

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### Обёртывание блоков кода в контейнер

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> теперь обёрнут внутри <div class="code-block">
```

## Поведение

- **Неизменяемость:** возвращается новое дерево; исходное не модифицируется
- **Обход сверху вниз:** посетитель вызывается для родителя до посещения его потомков
- **Замена:** если посетитель возвращает узел, он заменяет текущий узел до обхода его потомков
- **Без изменений:** если посетитель возвращает `void` (или `undefined`), исходный узел сохраняется
- **Рекурсия:** потомки узлов `"root"` и `"element"` обходятся рекурсивно
