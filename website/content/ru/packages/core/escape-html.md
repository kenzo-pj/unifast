---
title: "escapeHtml()"
description: "Экранирует специальные HTML-символы в строке."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Сигнатура

```ts
function escapeHtml(str: string): string
```

## Параметры

### str

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `str` | `string` | — | Строка, содержащая символы для экранирования |

## Возвращаемое значение

`string` — входная строка, в которой `&`, `<`, `>` и `"` заменены на соответствующие HTML-сущности.

## Использование

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Примеры

### Базовое экранирование

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Экранирование пользовательского контента

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Построение безопасных HTML-атрибутов

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Поведение

- **`&`** заменяется на `&amp;`
- **`<`** заменяется на `&lt;`
- **`>`** заменяется на `&gt;`
- **`"`** заменяется на `&quot;`
- Все остальные символы остаются без изменений
