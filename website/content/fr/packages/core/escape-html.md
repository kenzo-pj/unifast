---
title: "escapeHtml()"
description: "Échappe les caractères spéciaux HTML dans une chaîne."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Signature

```ts
function escapeHtml(str: string): string
```

## Paramètres

### str

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `str` | `string` | — | La chaîne contenant les caractères à échapper |

## Valeur de retour

`string` — La chaîne d'entrée dans laquelle `&`, `<`, `>` et `"` ont été remplacés par leurs entités HTML équivalentes.

## Utilisation

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Exemples

### Échappement de base

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Échappement de contenu généré par l'utilisateur

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Construction d'attributs HTML sûrs

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Comportement

- **`&`** est remplacé par `&amp;`
- **`<`** est remplacé par `&lt;`
- **`>`** est remplacé par `&gt;`
- **`"`** est remplacé par `&quot;`
- Tous les autres caractères sont conservés tels quels
