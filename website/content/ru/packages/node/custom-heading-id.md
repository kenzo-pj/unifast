---
title: "customHeadingId()"
description: "Задаёт пользовательские ID заголовков с помощью синтаксиса {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Сигнатура

```ts
function customHeadingId(): UnifastPlugin
```

## Параметры

Отсутствуют.

## Использование

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// Заголовок получает id="intro" вместо автоматически сгенерированного слага
```

## Примеры

### Пользовательский ID

Используйте синтаксис `{#custom-id}` в конце заголовка, чтобы назначить конкретный атрибут `id`. Блок в фигурных скобках удаляется из отрендеренного текста:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Заголовок без пользовательского ID

Заголовки без синтаксиса `{#...}` остаются без изменений. Они используют слаг по умолчанию, сгенерированный из текста заголовка:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Классы и произвольные атрибуты

Синтаксис фигурных скобок также поддерживает нотации `.class` и `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
