---
title: "visitHast()"
description: "Duyệt và biến đổi một cây HAST bằng hàm visitor."
---

```ts
import { visitHast } from "@unifast/core";
```

## Chữ ký

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Tham số

### node

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `type` | `string` | — | Loại nút (`"root"`, `"element"`, `"text"`, v.v.) |
| `children` | `HastNode[]` | — | Các nút con (dành cho loại `"root"` và `"element"`) |

### visitor

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | Hàm được gọi cho mỗi nút; trả về một nút để thay thế nút gốc, hoặc `void` để giữ nguyên |

## Giá trị trả về

`HastNode` — Một cây mới với các phép biến đổi được áp dụng bởi hàm visitor.

## Cách dùng

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

## Ví dụ

### Thêm class cho tất cả các đoạn văn

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

### Loại bỏ tất cả hình ảnh

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

### Thu thập tất cả liên kết

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

### Bọc khối code trong một container

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

## Hành vi

- **Immutable:** Trả về một cây mới; cây gốc không bị thay đổi
- **Duyệt từ trên xuống:** Visitor được gọi trên nút cha trước khi duyệt các nút con
- **Thay thế:** Nếu visitor trả về một nút, nó sẽ thay thế nút hiện tại trước khi duyệt các nút con
- **No-op:** Nếu visitor trả về `void` (hoặc `undefined`), nút gốc được giữ nguyên
- **Đệ quy:** Các nút con của `"root"` và `"element"` được duyệt đệ quy
