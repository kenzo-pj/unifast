---
title: "commentRemoval()"
description: "Rimuove i commenti HTML dall'output."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Firma

```ts
function commentRemoval(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

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

## Esempi

### Rimozione di base dei commenti

Tutti i nodi di commento HTML (`<!-- ... -->`) vengono rimossi dall'albero di output, inclusi i commenti annidati all'interno di elementi di blocco come le blockquote:

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

### L'HTML che non sia un commento viene preservato

Solo i nodi di commento vengono rimossi. L'altro HTML inline resta invariato:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
