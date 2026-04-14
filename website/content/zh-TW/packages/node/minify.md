---
title: "minify()"
description: "移除不必要的空白以縮小 HTML 輸出內容。"
---

```ts
import { minify } from "@unifast/node";
```

## 函式簽名

```ts
function minify(): UnifastPlugin
```

## 參數

無。

## 用法

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## 範例

### 基本的 minification

`minify()` 外掛會將連續的空白字元合併為單一空格、移除 HTML 註解、去除區塊元素間僅含空白的文字節點，並且移除空白的 `class` 與 `style` 屬性：

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### 保留預先格式化內容

`<pre>` 與 `<code>` 區塊內的空白會保持原樣，因此程式碼的排版不會被破壞：

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
