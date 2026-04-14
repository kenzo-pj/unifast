---
title: "abbr()"
description: "Преобразует определения аббревиатур в элементы <abbr> с атрибутами title."
---

```ts
import { abbr } from "@unifast/node";
```

## Сигнатура

```ts
function abbr(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Вхождения "HTML" оборачиваются в <abbr title="Hyper Text Markup Language">
```

## Примеры

### Базовое использование аббревиатур

Определите аббревиатуру синтаксисом `*[ТЕРМИН]: Определение`. Абзац с определением удаляется из вывода, а все вхождения термина оборачиваются в элементы `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Несколько аббревиатур

Вы можете определить несколько аббревиатур. Каждый термин заменяется независимо по всему документу:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
