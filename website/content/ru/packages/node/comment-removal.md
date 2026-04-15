---
title: "commentRemoval()"
description: "Удаляет HTML-комментарии из вывода."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Сигнатура

```ts
function commentRemoval(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// HTML-комментарий удаляется из вывода
```

## Примеры

### Базовое удаление комментариев

Все узлы HTML-комментариев (`<!-- ... -->`) удаляются из дерева вывода, включая комментарии, вложенные в блочные элементы, такие как цитаты:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### HTML, не являющийся комментарием, сохраняется

Удаляются только узлы комментариев. Остальной инлайн-HTML остаётся нетронутым:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
