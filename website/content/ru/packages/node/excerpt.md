---
title: "excerpt()"
description: "Извлекает отрывок из содержимого документа."
---

```ts
import { excerpt } from "@unifast/node";
```

## Сигнатура

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Параметры

### options?

Конфигурация извлечения отрывка

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Маркер-комментарий, отделяющий отрывок от остальной части |
| `fallbackParagraphs` | `number` | `1` | Количество первых абзацев, используемых как отрывок, если разделитель не найден |
| `fallbackCharacters` | `number` | `undefined` | Максимальная длина отрывка в символах (обрезается по границе слова) |

Если одновременно заданы `fallbackParagraphs` и `fallbackCharacters`, приоритет имеет `fallbackParagraphs`.

## Возвращаемое значение

Плагин добавляет свойство `excerpt` в результат компиляции:

| Свойство | Тип | Описание |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Текстовый отрывок, извлечённый из документа |

## Использование

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Примеры

### С маркером-разделителем

Поместите комментарий `<!-- more -->` в Markdown, чтобы явно обозначить конец отрывка:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### Запасной вариант — первый абзац

Если маркер-разделитель не найден, плагин возвращает первые N абзацев:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Запасной вариант — ограничение по символам

Обрезайте отрывок до максимальной длины в символах с разрывом по границе слова:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
