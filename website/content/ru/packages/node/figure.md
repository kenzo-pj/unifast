---
title: "figure()"
description: "Оборачивает изображения с альтернативным текстом в элементы <figure> и <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Сигнатура

```ts
function figure(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// Изображение оборачивается в <figure> с <figcaption>
```

## Примеры

### Базовое обёртывание в figure

Когда у изображения есть альтернативный текст, `figure()` оборачивает его в элемент `<figure>` и добавляет `<figcaption>` с этим альтернативным текстом:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Изображение без альтернативного текста

Изображения без альтернативного текста не оборачиваются, так как отсутствует содержательная подпись для отображения:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
