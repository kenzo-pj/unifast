---
title: "findCodeChild()"
description: "Tìm phần tử <code> con bên trong một phần tử cha."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Chữ ký

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Tham số

### element

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Định danh loại nút |
| `tagName` | `string` | — | Tên thẻ HTML (thường là `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Các thuộc tính của phần tử |
| `children` | `HastNode[]` | — | Các nút con để tìm kiếm |

## Giá trị trả về

`HastElement | undefined` — Phần tử con đầu tiên có `tagName` là `"code"`, hoặc `undefined` nếu không tồn tại phần tử con nào như vậy.

## Cách dùng

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

## Ví dụ

### Tìm code bên trong phần tử pre

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

### Khi không có phần tử code con

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

### Kết hợp với visitHast để syntax highlighting

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
