---
title: "extractText()"
description: "遞迴擷取 HAST 節點中的所有文字內容。"
---

```ts
import { extractText } from "@unifast/core";
```

## 函式簽名

```ts
function extractText(node: HastNode): string
```

## 參數

### node

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `type` | `string` | — | 節點型別（`"root"`、`"element"`、`"text"` 等） |
| `children` | `HastNode[]` | — | 子節點（適用於 `"root"` 與 `"element"` 型別） |
| `value` | `string` | — | 文字內容（適用於 `"text"` 型別） |

## 回傳值

`string` — 從該節點及其所有子節點串接而成的全部文字內容。

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

## 範例

### 從簡單元素擷取

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

### 從巢狀元素擷取

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

### 產生標題 slug

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
