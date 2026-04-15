---
title: "codeMeta()"
description: "將程式碼圍籬的 meta 字串解析為程式碼區塊上的 data 屬性。"
---

```ts
import { codeMeta } from "@unifast/node";
```

## 函式簽名

```ts
function codeMeta(): UnifastPlugin
```

## 參數

無。

## 用法

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

## 範例

### 基本的 meta 解析

`codeMeta()` 外掛會解析圍籬式程式碼區塊中語言識別字之後的 meta 字串，並將可識別的鍵轉換為 `<pre>` 元素上的 `data-*` 屬性：

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

### 多個 meta 屬性

您可以組合多個 meta 屬性，例如 `title`、用於行號高亮的 `{1,3-5}`、`showLineNumbers`、`diff` 與 `wordWrap`：

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
