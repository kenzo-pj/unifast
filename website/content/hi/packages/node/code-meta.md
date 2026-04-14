---
title: "codeMeta()"
description: "code fence meta strings को code blocks पर data attributes में parse करें।"
---

```ts
import { codeMeta } from "@unifast/node";
```

## Signature

```ts
function codeMeta(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// <pre> element को data-title="example.js" मिल जाता है
```

## उदाहरण

### मूल meta parsing

`codeMeta()` plugin fenced code blocks में language identifier के बाद के meta string को parse करता है और पहचानी गई keys को `<pre>` element पर `data-*` attributes में convert करता है:

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

### कई meta attributes

आप `title`, line highlighting के लिए `{1,3-5}`, `showLineNumbers`, `diff`, और `wordWrap` जैसे कई meta attributes को combine कर सकते हैं:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// <pre> element को मिलता है:
//   data-title="server.ts"
//   Lines 1 और 3 को data-highlighted attributes मिलते हैं
//   showLineNumbers एक boolean flag के रूप में पहचाना जाता है
```
