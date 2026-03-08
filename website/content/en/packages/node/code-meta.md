---
title: "codeMeta()"
description: "Parse code fence meta strings into data attributes on code blocks."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Signature

```ts
function codeMeta(): UnifastPlugin
```

## Parameters

None.

## Usage

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

## Examples

### Basic meta parsing

The `codeMeta()` plugin parses the meta string after the language identifier in fenced code blocks and converts recognized keys into `data-*` attributes on the `<pre>` element:

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

### Multiple meta attributes

You can combine multiple meta attributes such as `title`, `{1,3-5}` for line highlighting, `showLineNumbers`, `diff`, and `wordWrap`:

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
