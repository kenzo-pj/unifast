---
title: "findCodeChild()"
description: "在父元素内查找子 <code> 元素。"
---

```ts
import { findCodeChild } from "@unifast/core";
```

## 签名

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## 参数

### element

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 节点类型标识 |
| `tagName` | `string` | — | HTML 标签名（通常是 `"pre"`） |
| `properties` | `Record<string, unknown>` | — | 元素属性 |
| `children` | `HastNode[]` | — | 要检索的子节点 |

## 返回值

`HastElement | undefined` —— 第一个 `tagName` 为 `"code"` 的子元素；若不存在则返回 `undefined`。

## 用法

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-js"] },
      children: [{ type: "text", value: "const x = 1;" }],
    },
  ],
};

const code = findCodeChild(pre);

console.log(code?.tagName);
// code
```

## 示例

### 在 pre 元素中查找 code

```ts
import { findCodeChild, extractLang, extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-rust"] },
      children: [{ type: "text", value: 'fn main() { println!("hello"); }' }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // rust
  console.log(extractText(code));
  // fn main() { println!("hello"); }
}
```

### 不存在 code 子元素的情况

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    { type: "text", value: "plain preformatted text" },
  ],
};

const code = findCodeChild(pre);

console.log(code);
// undefined
```

### 搭配 visitHast 实现语法高亮

```ts
import { visitHast, findCodeChild, extractLang } from "@unifast/core";
import type { HastNode, HastElement } from "@unifast/core";

const tree: HastNode = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    const code = findCodeChild(node);
    if (code) {
      const lang = extractLang(code);
      console.log(`Found code block with language: ${lang}`);
      // Found code block with language: js
    }
  }
});
```
