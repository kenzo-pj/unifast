---
title: "escapeHtml()"
description: "Effettua l'escape dei caratteri speciali HTML in una stringa."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Firma

```ts
function escapeHtml(str: string): string
```

## Parametri

### str

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `str` | `string` | — | La stringa che contiene i caratteri di cui fare l'escape |

## Valore restituito

`string` — La stringa di input con `&`, `<`, `>` e `"` sostituiti dalle rispettive entità HTML.

## Utilizzo

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Esempi

### Escape di base

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Escape di contenuti generati dall'utente

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Costruzione di attributi HTML sicuri

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Comportamento

- **`&`** viene sostituito con `&amp;`
- **`<`** viene sostituito con `&lt;`
- **`>`** viene sostituito con `&gt;`
- **`"`** viene sostituito con `&quot;`
- Tutti gli altri caratteri restano invariati
