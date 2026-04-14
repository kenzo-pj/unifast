---
title: "codeMeta()"
description: "Parst Meta-Strings von Code-Fences in data-Attribute auf Code-Blöcken."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Signatur

```ts
function codeMeta(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

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

## Beispiele

### Grundlegendes Meta-Parsing

Das `codeMeta()`-Plugin parst den Meta-String nach dem Sprachbezeichner in eingezäunten Code-Blöcken und konvertiert erkannte Schlüssel in `data-*`-Attribute auf dem `<pre>`-Element:

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

### Mehrere Meta-Attribute

Sie können mehrere Meta-Attribute kombinieren, etwa `title`, `{1,3-5}` zur Zeilen-Hervorhebung, `showLineNumbers`, `diff` und `wordWrap`:

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
