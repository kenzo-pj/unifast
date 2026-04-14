---
title: "accessibleEmoji()"
description: 'Оборачивает символы эмодзи в элементы <span role="img"> с атрибутами aria-label для доступности.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Сигнатура

```ts
function accessibleEmoji(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Эмодзи оборачивается в <span role="img" aria-label="rocket">
```

## Примеры

### Оборачивание эмодзи с aria-метками

Каждый символ эмодзи оборачивается в `<span>` с `role="img"` и `aria-label`, описывающим эмодзи для программ чтения с экрана:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Текст без эмодзи

Простой текст без символов эмодзи проходит без изменений:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
