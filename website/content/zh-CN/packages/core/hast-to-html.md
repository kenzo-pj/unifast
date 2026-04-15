---
title: "hastToHtml()"
description: "将 HAST 根节点序列化为 HTML 字符串。"
---

```ts
import { hastToHtml } from "@unifast/core";
```

## 签名

```ts
function hastToHtml(hast: HastRoot): string
```

## 参数

### hast

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `type` | `"root"` | — | 节点类型标识 |
| `children` | `HastNode[]` | — | 树的子节点 |

## 返回值

`string` —— 序列化后的 HTML 字符串。

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

## 示例

### 基础序列化

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

### 空元素

空元素（`<br>`、`<img>`、`<hr>` 等）会自动以自闭合形式输出：

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

### 配合 compile() 输出使用

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### 原始 HTML 透传

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

## 行为说明

- **HTML 转义：** 文本内容会被转义（`&`、`<`、`>`、`"`）
- **空元素：** `area`、`base`、`br`、`col`、`embed`、`hr`、`img`、`input`、`link`、`meta`、`param`、`source`、`track`、`wbr` 会以自闭合形式输出
- **属性：** 按字母顺序排序；`className` 数组会用空格拼接并以 `class` 属性渲染；布尔 `true` 渲染为不带值的属性；`false`/`null`/`undefined` 会被省略
- **注释：** 渲染为 `<!--value-->`
- **Doctype：** 渲染为 `<!DOCTYPE html>`
- **Raw 节点：** 原样输出，不进行转义
