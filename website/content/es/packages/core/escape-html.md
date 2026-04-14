---
title: "escapeHtml()"
description: "Escapa los caracteres especiales de HTML en una cadena."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Firma

```ts
function escapeHtml(str: string): string
```

## Parámetros

### str

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `str` | `string` | — | La cadena que contiene los caracteres a escapar |

## Retorna

`string` — La cadena de entrada con `&`, `<`, `>` y `"` reemplazados por sus entidades HTML equivalentes.

## Uso

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Ejemplos

### Escape básico

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Escape de contenido generado por el usuario

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Construcción de atributos HTML seguros

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Comportamiento

- **`&`** se reemplaza por `&amp;`
- **`<`** se reemplaza por `&lt;`
- **`>`** se reemplaza por `&gt;`
- **`"`** se reemplaza por `&quot;`
- El resto de los caracteres se mantienen sin cambios
