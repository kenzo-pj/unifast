---
title: "codeMeta()"
description: "Разбирает мета-строки огороженных блоков кода в data-атрибуты на элементах code."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Сигнатура

```ts
function codeMeta(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// Элемент <pre> получает data-title="example.js"
```

## Примеры

### Базовый разбор мета-строки

Плагин `codeMeta()` разбирает мета-строку после идентификатора языка в огороженных блоках кода и преобразует распознанные ключи в атрибуты `data-*` элемента `<pre>`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="app.ts"
const x = 1;
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
console.log(result.output);
// <pre data-lang="js" data-title="app.ts"><code class="language-js">const x = 1;
// </code></pre>
```

### Несколько мета-атрибутов

Вы можете комбинировать несколько мета-атрибутов, таких как `title`, `{1,3-5}` для выделения строк, `showLineNumbers`, `diff` и `wordWrap`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// Элемент <pre> получает:
//   data-title="server.ts"
//   Строки 1 и 3 получают атрибуты data-highlighted
//   showLineNumbers распознаётся как булев флаг
```
