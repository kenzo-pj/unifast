---
title: "codeMeta()"
description: "Analyse les chaînes meta des blocs de code clôturés et les convertit en attributs data sur ces blocs."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Signature

```ts
function codeMeta(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

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

## Exemples

### Analyse meta de base

Le plugin `codeMeta()` analyse la chaîne meta placée après l'identifiant de langage dans les blocs de code clôturés et convertit les clés reconnues en attributs `data-*` sur l'élément `<pre>` :

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

### Attributs meta multiples

Vous pouvez combiner plusieurs attributs meta, tels que `title`, `{1,3-5}` pour la mise en évidence de lignes, `showLineNumbers`, `diff` et `wordWrap` :

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
