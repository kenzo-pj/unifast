---
title: "visitHast()"
description: "通过访问者函数遍历并转换 HAST 树。"
---

```ts
import { visitHast } from "@unifast/core";
```

## 签名

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## 参数

### node

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `type` | `string` | — | 节点类型（`"root"`、`"element"`、`"text"` 等） |
| `children` | `HastNode[]` | — | 子节点（用于 `"root"` 和 `"element"` 类型） |

### visitor

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | 对每个节点调用的函数；返回新节点可替换原节点，返回 `void` 则保留原节点 |

## 返回值

`HastNode` —— 应用 visitor 转换后的新树。

## 用法

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// Tree with text "HELLO WORLD"
```

## 示例

### 为所有段落添加类名

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// Both <p> elements now have className: ["prose"]
```

### 移除所有图片

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// The <img> element has been removed from the tree
```

### 收集所有链接

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### 将代码块包裹在容器中

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> is now wrapped inside <div class="code-block">
```

## 行为说明

- **不可变：** 返回一棵新树，原树不会被修改
- **自顶向下遍历：** visitor 先对父节点调用，然后再访问其子节点
- **替换：** 如果 visitor 返回一个节点，该节点会在子节点被遍历之前替换掉当前节点
- **空操作：** 如果 visitor 返回 `void`（或 `undefined`），则保留原节点
- **递归：** `"root"` 和 `"element"` 节点的子节点会被递归访问
