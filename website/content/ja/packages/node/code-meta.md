---
title: "codeMeta()"
description: "コードフェンスのメタ文字列をパースして、コードブロックのデータ属性へ変換します。"
---

```ts
import { codeMeta } from "@unifast/node";
```

## シグネチャ

```ts
function codeMeta(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

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

## 使用例

### 基本的なメタのパース

`codeMeta()` プラグインは、フェンスドコードブロックの言語識別子の後に続くメタ文字列をパースし、認識したキーを `<pre>` 要素の `data-*` 属性へ変換します。

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

### 複数のメタ属性

`title`、行ハイライト用の `{1,3-5}`、`showLineNumbers`、`diff`、`wordWrap` など、複数のメタ属性を組み合わせられます。

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
