---
title: "extractText()"
description: "递归提取 HAST 节点中的所有文本内容。"
---

```ts
import { extractText } from "@unifast/core";
```

## 签名

```ts
function extractText(node: HastNode): string
```

## 参数

### node

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `type` | `string` | — | 节点类型（`"root"`、`"element"`、`"text"` 等） |
| `children` | `HastNode[]` | — | 子节点（用于 `"root"` 和 `"element"` 类型） |
| `value` | `string` | — | 文本内容（用于 `"text"` 类型） |

## 返回值

`string` —— 将该节点及其所有后代节点的文本内容拼接起来得到的字符串。

## 用法

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const element: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "Hello " },
    {
      type: "element",
      tagName: "strong",
      properties: {},
      children: [{ type: "text", value: "world" }],
    },
  ],
};

const text = extractText(element);

console.log(text);
// Hello world
```

## 示例

### 从简单元素中提取文本

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h1",
  properties: { id: "title" },
  children: [{ type: "text", value: "Getting Started" }],
};

console.log(extractText(heading));
// Getting Started
```

### 从嵌套元素中提取文本

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const paragraph: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "This is " },
    {
      type: "element",
      tagName: "em",
      properties: {},
      children: [
        { type: "text", value: "deeply " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "nested" }],
        },
      ],
    },
    { type: "text", value: " content." },
  ],
};

console.log(extractText(paragraph));
// This is deeply nested content.
```

### 空元素

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const emptyDiv: HastElement = {
  type: "element",
  tagName: "div",
  properties: {},
  children: [],
};

console.log(extractText(emptyDiv));
// (empty string)
```

### 生成标题 slug

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h2",
  properties: {},
  children: [
    { type: "text", value: "API " },
    {
      type: "element",
      tagName: "code",
      properties: {},
      children: [{ type: "text", value: "Reference" }],
    },
  ],
};

const slug = extractText(heading).toLowerCase().replace(/\s+/g, "-");

console.log(slug);
// api-reference
```
