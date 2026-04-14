---
title: "codeMeta()"
description: "Kod fence meta string'lerini kod bloklarındaki data özniteliklerine ayrıştırır."
---

```ts
import { codeMeta } from "@unifast/node";
```

## İmza

```ts
function codeMeta(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// <pre> elemanı data-title="example.js" özniteliğini alır
```

## Örnekler

### Temel meta ayrıştırma

`codeMeta()` plugin'i, fenced kod bloklarındaki dil tanımlayıcısından sonra gelen meta string'i ayrıştırır ve tanınan anahtarları `<pre>` elemanındaki `data-*` özniteliklerine dönüştürür:

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

### Birden fazla meta özniteliği

`title`, satır vurgulama için `{1,3-5}`, `showLineNumbers`, `diff` ve `wordWrap` gibi birden fazla meta özniteliğini birleştirebilirsiniz:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// <pre> elemanı şunları alır:
//   data-title="server.ts"
//   1. ve 3. satırlar data-highlighted özniteliklerini alır
//   showLineNumbers boolean bir bayrak olarak tanınır
```
