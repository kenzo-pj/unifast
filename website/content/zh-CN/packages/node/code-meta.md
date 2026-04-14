---
title: "codeMeta()"
description: "将代码围栏的 meta 字符串解析为代码块上的 data 属性。"
---

```ts
import { codeMeta } from "@unifast/node";
```

## 签名

```ts
function codeMeta(): UnifastPlugin
```

## 参数

无。

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

## 示例

### 基础 meta 解析

`codeMeta()` 插件会解析围栏代码块中语言标识之后的 meta 字符串，并将可识别的键转换为 `<pre>` 元素上的 `data-*` 属性：

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

### 多个 meta 属性

你可以组合使用多种 meta 属性，例如 `title`、用于行高亮的 `{1,3-5}`、`showLineNumbers`、`diff` 和 `wordWrap`：

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
