---
title: "escapeHtml()"
description: "Escapa caracteres especiais HTML em uma string."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Assinatura

```ts
function escapeHtml(str: string): string
```

## Parâmetros

### str

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `str` | `string` | — | A string contendo caracteres a serem escapados |

## Retorna

`string` — A string de entrada com `&`, `<`, `>` e `"` substituídos por seus equivalentes em entidades HTML.

## Uso

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Exemplos

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

### Escape de conteúdo gerado pelo usuário

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Construindo atributos HTML seguros

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Comportamento

- **`&`** é substituído por `&amp;`
- **`<`** é substituído por `&lt;`
- **`>`** é substituído por `&gt;`
- **`"`** é substituído por `&quot;`
- Todos os outros caracteres são deixados inalterados
