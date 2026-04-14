---
title: "codeMeta()"
description: "Mem-parse string meta dari code fence menjadi atribut data pada code block."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Signature

```ts
function codeMeta(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

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

## Contoh

### Parsing meta dasar

Plugin `codeMeta()` mem-parse string meta setelah identifier bahasa di fenced code block dan mengonversi key yang dikenali menjadi atribut `data-*` pada elemen `<pre>`:

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

### Beberapa atribut meta

Anda dapat menggabungkan beberapa atribut meta seperti `title`, `{1,3-5}` untuk highlighting baris, `showLineNumbers`, `diff`, dan `wordWrap`:

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
