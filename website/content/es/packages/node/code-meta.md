---
title: "codeMeta()"
description: "Parsea las cadenas meta de los bloques de código fenced y las convierte en atributos data en los bloques de código."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Firma

```ts
function codeMeta(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// The <pre> element gets data-title="example.js"
```

## Ejemplos

### Parsing meta básico

El plugin `codeMeta()` parsea la cadena meta que sigue al identificador de lenguaje en los bloques de código fenced y convierte las claves reconocidas en atributos `data-*` sobre el elemento `<pre>`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="app.ts"
const x = 1;
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
console.log(result.output);
// <pre data-lang="js" data-title="app.ts"><code class="language-js">const x = 1;
// </code></pre>
```

### Múltiples atributos meta

Puedes combinar múltiples atributos meta como `title`, `{1,3-5}` para el resaltado de líneas, `showLineNumbers`, `diff` y `wordWrap`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// The <pre> element receives:
//   data-title="server.ts"
//   Lines 1 and 3 get data-highlighted attributes
//   showLineNumbers is recognized as a boolean flag
```
