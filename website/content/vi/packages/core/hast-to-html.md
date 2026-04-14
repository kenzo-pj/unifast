---
title: "hastToHtml()"
description: "Tuần tự hóa một nút gốc HAST thành chuỗi HTML."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## Chữ ký

```ts
function hastToHtml(hast: HastRoot): string
```

## Tham số

### hast

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `type` | `"root"` | — | Định danh loại nút |
| `children` | `HastNode[]` | — | Các nút con của cây |

## Giá trị trả về

`string` — Chuỗi HTML đã được tuần tự hóa.

## Cách dùng

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

## Ví dụ

### Tuần tự hóa cơ bản

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

### Phần tử void

Các phần tử void (`<br>`, `<img>`, `<hr>`, v.v.) được tự động tự đóng:

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

### Kết hợp với đầu ra của compile()

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### HTML thô đi qua không xử lý

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

## Hành vi

- **Escape HTML:** Nội dung văn bản được escape (`&`, `<`, `>`, `"`)
- **Phần tử void:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr` được tự đóng
- **Thuộc tính:** Được sắp xếp theo thứ tự bảng chữ cái; mảng `className` được nối bằng dấu cách và render thành `class`; thuộc tính boolean `true` render dưới dạng thuộc tính trần; `false`/`null`/`undefined` bị bỏ qua
- **Comment:** Render thành `<!--value-->`
- **Doctype:** Render thành `<!DOCTYPE html>`
- **Nút raw:** Xuất ra nguyên trạng, không escape
