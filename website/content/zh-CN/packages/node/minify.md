---
title: "minify()"
description: "通过移除多余空白来压缩 HTML 输出。"
---

```ts
import { minify } from "@unifast/node";
```

## 签名

```ts
function minify(): UnifastPlugin
```

## 参数

无。

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

## 示例

### 基础压缩

`minify()` 插件会将连续的空白字符折叠为单个空格，移除 HTML 注释，剥离块级元素之间仅包含空白的文本节点，并移除空的 `class` 和 `style` 属性：

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

### 预格式化内容会被保留

`<pre>` 和 `<code>` 块内部的空白不会被改动，因此代码的排版不会被破坏：

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
