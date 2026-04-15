---
title: "commentRemoval()"
description: "從輸出中移除 HTML 註解。"
---

```ts
import { commentRemoval } from "@unifast/node";
```

## 函式簽名

```ts
function commentRemoval(): UnifastPlugin
```

## 參數

無。

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

## 範例

### 基本的註解移除

所有 HTML 註解節點（`<!-- ... -->`）都會從輸出樹中移除，包含巢狀於區塊元素（如引用區塊）之中的註解：

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

### 非註解的 HTML 會保留

只有註解節點會被移除，其他行內 HTML 則保持不變：

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
