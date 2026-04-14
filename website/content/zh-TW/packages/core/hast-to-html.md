---
title: "hastToHtml()"
description: "將 HAST 根節點序列化為 HTML 字串。"
---

```ts
import { hastToHtml } from "@unifast/core";
```

## 函式簽名

```ts
function hastToHtml(hast: HastRoot): string
```

## 參數

### hast

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `type` | `"root"` | — | 節點型別識別字 |
| `children` | `HastNode[]` | — | 樹的子節點 |

## 回傳值

`string` — 序列化後的 HTML 字串。

## 用法

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## 範例

### 基本序列化

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### 空元素（void elements）

空元素（`<br>`、`<img>`、`<hr>` 等）會自動以自我結束（self-closed）的形式輸出：

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### 搭配 compile() 的輸出

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### 原始 HTML 直通

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## 行為

- **HTML 跳脫：** 文字內容會被跳脫（`&`、`<`、`>`、`"`）
- **空元素：** `area`、`base`、`br`、`col`、`embed`、`hr`、`img`、`input`、`link`、`meta`、`param`、`source`、`track`、`wbr` 會以自我結束形式輸出
- **屬性：** 會依字母順序排序；`className` 陣列會以空白字元串接並輸出為 `class`；布林值 `true` 會輸出為不帶值的屬性；`false`、`null`、`undefined` 則會被省略
- **註解：** 會輸出為 `<!--value-->`
- **Doctype：** 會輸出為 `<!DOCTYPE html>`
- **Raw 節點：** 會原封不動地輸出，不會進行跳脫
