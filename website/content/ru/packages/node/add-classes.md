---
title: "addClasses()"
description: "Добавляет CSS-классы к элементам, соответствующим CSS-селекторам."
---

```ts
import { addClasses } from "@unifast/node";
```

## Сигнатура

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Параметры

### rules

`Record<string, string>`, где ключи — это CSS-селекторы, а значения — имена классов, разделённые пробелами, которые добавляются к соответствующим элементам. Классы объединяются с уже имеющимися классами элемента.

### Поддерживаемые селекторы

Поддерживается широкое подмножество CSS Selectors Level 4, в том числе:

- **Селекторы тегов**: `h1`, `p`, `table`
- **Селекторы классов**: `.info`, `.alert.warning`
- **Селекторы ID**: `#main`
- **Универсальный селектор**: `*`
- **Селекторы атрибутов**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Комбинаторы**: потомок (` `), дочерний (`>`), соседний (`+`), общий родственный (`~`)
- **Псевдоклассы**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Селекторы через запятую**: `h1, h2, h3`
- **Составные селекторы**: `div.alert#main[data-type="warning"]`

## Использование

```ts
import { compile, addClasses } from "@unifast/node";

const md = `
# Hello World

Some paragraph text.

| Name | Value |
|------|-------|
| A    | 1     |
`;

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-3xl font-bold",
      p: "leading-relaxed",
      table: "border-collapse w-full",
    }),
  ],
});

// <h1 class="text-3xl font-bold">Hello World</h1>
// <p class="leading-relaxed">Some paragraph text.</p>
// <table class="border-collapse w-full">...</table>
```

## Примеры

### Селекторы через запятую

Применяйте одни и те же классы к нескольким типам элементов:

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "h1, h2, h3": "font-bold tracking-tight",
    }),
  ],
});
```

### Сложные селекторы

Используйте комбинаторы и псевдоклассы для точного нацеливания:

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "pre > code": "block overflow-x-auto",
      "ul > li:first-child": "mt-0",
      "ul > li:last-child": "mb-0",
      "a[href^=\"https\"]": "external-link",
      "div:not(.alert)": "default-container",
    }),
  ],
});
```

### Утилитарные классы Tailwind CSS

Типичный приём — использовать `addClasses` для применения утилит Tailwind к HTML, сгенерированному из Markdown:

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-4xl font-extrabold text-gray-900 mb-8",
      h2: "text-2xl font-bold text-gray-800 mt-12 mb-4",
      h3: "text-xl font-semibold text-gray-700 mt-8 mb-3",
      p: "text-base leading-7 text-gray-600 mb-4",
      a: "text-blue-600 underline hover:text-blue-800",
      blockquote: "border-l-4 border-gray-300 pl-4 italic text-gray-500",
      table: "min-w-full divide-y divide-gray-200",
      "thead th": "px-4 py-2 text-left text-sm font-semibold text-gray-900",
      "tbody td": "px-4 py-2 text-sm text-gray-700",
      "tbody tr:nth-child(2n)": "bg-gray-50",
      img: "rounded-lg shadow-md",
      pre: "rounded-lg overflow-hidden",
      "pre > code": "block p-4 text-sm",
    }),
  ],
});
```

### Селекторы атрибутов

Нацеливайтесь на элементы по их атрибутам:

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "[href$=\".pdf\"]": "pdf-link",
      "[href^=\"https\"]": "external",
      "[data-type]": "has-type",
    }),
  ],
});
```
