---
title: "commentRemoval()"
description: "Elimina los comentarios HTML de la salida."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Firma

```ts
function commentRemoval(): UnifastPlugin
```

## Parámetros

Ninguno.

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

## Ejemplos

### Eliminación básica de comentarios

Todos los nodos de comentario HTML (`<!-- ... -->`) se eliminan del árbol de salida, incluidos los comentarios anidados dentro de elementos en bloque como los blockquotes:

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

### El HTML que no es comentario se conserva

Solo se eliminan los nodos de comentario. El resto del HTML en línea queda intacto:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
