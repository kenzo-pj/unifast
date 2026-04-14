---
title: "findCodeChild()"
description: "在父元素中尋找 <code> 子元素。"
---

```ts
import { findCodeChild } from "@unifast/core";
```

## 函式簽名

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## 參數

### element

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 節點型別識別字 |
| `tagName` | `string` | — | HTML 標籤名稱（通常為 `"pre"`） |
| `properties` | `Record<string, unknown>` | — | 元素屬性 |
| `children` | `HastNode[]` | — | 要搜尋的子節點 |

## 回傳值

`HastElement | undefined` — 第一個 `tagName` 為 `"code"` 的子元素；若不存在則為 `undefined`。

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

## 範例

### 在 pre 元素中尋找 code

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

### 沒有 code 子元素時

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

### 搭配 visitHast 進行語法高亮

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
