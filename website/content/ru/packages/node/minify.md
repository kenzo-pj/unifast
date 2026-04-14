---
title: "minify()"
description: "Минифицирует HTML-вывод, удаляя ненужные пробелы."
---

```ts
import { minify } from "@unifast/node";
```

## Сигнатура

```ts
function minify(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Пробелы сворачиваются, ненужные узлы удаляются
```

## Примеры

### Базовая минификация

Плагин `minify()` сворачивает последовательные пробельные символы в одиночные пробелы, удаляет HTML-комментарии, убирает текстовые узлы, состоящие только из пробелов, между блочными элементами, а также удаляет пустые атрибуты `class` и `style`:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Предварительно отформатированное содержимое сохраняется

Пробелы внутри блоков `<pre>` и `<code>` остаются нетронутыми, поэтому форматирование кода никогда не нарушается:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Пробелы внутри блока <pre><code> сохраняются ровно так, как были записаны
```
