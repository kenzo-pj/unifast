---
title: "visitHast()"
description: "使用 visitor 函式走訪並轉換 HAST 樹。"
---

```ts
import { visitHast } from "@unifast/core";
```

## 函式簽名

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## 參數

### node

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `type` | `string` | — | 節點型別（`"root"`、`"element"`、`"text"` 等） |
| `children` | `HastNode[]` | — | 子節點（適用於 `"root"` 與 `"element"` 型別） |

### visitor

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | 針對每個節點呼叫的函式；若回傳節點則會取代原節點，若回傳 `void` 則保留原節點 |

## 回傳值

`HastNode` — 套用了 visitor 函式所有轉換的新樹。

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

## 範例

### 為所有段落加上 class

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

### 移除所有圖片

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

### 蒐集所有連結

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

### 將程式碼區塊包在容器中

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

## 行為

- **不可變：** 回傳的是一棵新樹，原始樹不會被修改
- **由上而下走訪：** visitor 會在子節點走訪之前先對父節點呼叫
- **替換節點：** 若 visitor 回傳了節點，該節點會在子節點走訪之前取代目前節點
- **不做變更：** 若 visitor 回傳 `void`（或 `undefined`），則保留原本的節點
- **遞迴：** `"root"` 與 `"element"` 節點的子節點會被遞迴走訪
