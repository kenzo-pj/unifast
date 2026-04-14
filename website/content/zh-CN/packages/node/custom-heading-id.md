---
title: "customHeadingId()"
description: "通过 {#custom-id} 语法为标题设置自定义 ID。"
---

```ts
import { customHeadingId } from "@unifast/node";
```

## 签名

```ts
function customHeadingId(): UnifastPlugin
```

## 参数

无。

## 用法

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## 示例

### 自定义 ID

在标题末尾使用 `{#custom-id}` 语法即可指定 `id` 属性。花括号所在的部分会从渲染文本中移除：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### 未使用自定义 ID 的标题

不使用 `{#...}` 语法的标题会保持不变。它们会使用从标题文本生成的默认 slug：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### 类名和任意属性

花括号语法还支持 `.class` 以及 `key=value` 形式：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
