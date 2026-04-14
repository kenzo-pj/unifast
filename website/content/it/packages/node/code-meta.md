---
title: "codeMeta()"
description: "Analizza le stringhe meta delle code fence in attributi data sui blocchi di codice."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Firma

```ts
function codeMeta(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

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

## Esempi

### Parsing di meta di base

Il plugin `codeMeta()` analizza la stringa meta che segue l'identificatore di linguaggio nei blocchi di codice recintati e converte le chiavi riconosciute in attributi `data-*` sull'elemento `<pre>`:

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

### Attributi meta multipli

È possibile combinare più attributi meta come `title`, `{1,3-5}` per l'evidenziazione delle righe, `showLineNumbers`, `diff` e `wordWrap`:

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
