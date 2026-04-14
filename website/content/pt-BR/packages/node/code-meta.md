---
title: "codeMeta()"
description: "Faz parsing de strings meta de code fence em atributos data nos code blocks."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Assinatura

```ts
function codeMeta(): UnifastPlugin
```

## Parâmetros

Nenhum.

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

## Exemplos

### Parsing básico de meta

O plugin `codeMeta()` faz parsing da string meta após o identificador de linguagem em code blocks fenced e converte chaves reconhecidas em atributos `data-*` no elemento `<pre>`:

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

### Múltiplos atributos meta

Você pode combinar múltiplos atributos meta como `title`, `{1,3-5}` para highlighting de linhas, `showLineNumbers`, `diff` e `wordWrap`:

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
