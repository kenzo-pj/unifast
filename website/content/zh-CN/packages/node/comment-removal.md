---
title: "commentRemoval()"
description: "从输出中移除 HTML 注释。"
---

```ts
import { commentRemoval } from "@unifast/node";
```

## 签名

```ts
function commentRemoval(): UnifastPlugin
```

## 参数

无。

## 用法

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## 示例

### 基础注释移除

所有 HTML 注释节点（`<!-- ... -->`）都会从输出树中被剥离，包括嵌套在引用块等块级元素内部的注释：

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### 非注释 HTML 会被保留

只有注释节点会被移除。其他内联 HTML 会保持原样：

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
