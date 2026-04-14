---
title: "escapeHtml()"
description: "HTML-Sonderzeichen in einem String maskieren."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Signatur

```ts
function escapeHtml(str: string): string
```

## Parameter

### str

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `str` | `string` | — | Der zu maskierende String mit Zeichen, die escaped werden sollen |

## Rückgabewert

`string` – Der Eingabestring, bei dem `&`, `<`, `>` und `"` durch ihre HTML-Entity-Entsprechungen ersetzt wurden.

## Verwendung

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Beispiele

### Einfache Maskierung

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Maskierung benutzergenerierter Inhalte

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Sichere HTML-Attribute erstellen

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Verhalten

- **`&`** wird durch `&amp;` ersetzt
- **`<`** wird durch `&lt;` ersetzt
- **`>`** wird durch `&gt;` ersetzt
- **`"`** wird durch `&quot;` ersetzt
- Alle anderen Zeichen bleiben unverändert
