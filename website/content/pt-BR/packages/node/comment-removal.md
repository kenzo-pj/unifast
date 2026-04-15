---
title: "commentRemoval()"
description: "Remove comentários HTML da saída."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Assinatura

```ts
function commentRemoval(): UnifastPlugin
```

## Parâmetros

Nenhum.

## Uso

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## Exemplos

### Remoção básica de comentários

Todos os nós de comentário HTML (`<!-- ... -->`) são removidos da árvore de saída, incluindo comentários aninhados dentro de elementos block como blockquotes:

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

### HTML que não é comentário é preservado

Apenas nós de comentário são removidos. Outros HTMLs inline são mantidos intactos:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
