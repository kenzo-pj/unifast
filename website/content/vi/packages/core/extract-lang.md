---
title: "extractLang()"
description: "Trích xuất ngôn ngữ lập trình từ className của một phần tử code."
---

```ts
import { extractLang } from "@unifast/core";
```

## Chữ ký

```ts
function extractLang(code: HastElement): string | null
```

## Tham số

### code

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Định danh loại nút |
| `tagName` | `string` | — | Tên thẻ HTML (thường là `"code"`) |
| `properties` | `Record<string, unknown>` | — | Các thuộc tính của phần tử, bao gồm cả `className` |
| `children` | `HastNode[]` | — | Các nút con của phần tử |

## Giá trị trả về

`string | null` — Định danh ngôn ngữ được trích xuất từ class `language-*` đầu tiên, hoặc `null` nếu không tìm thấy class nào về ngôn ngữ.

## Cách dùng

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const codeElement: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-typescript"] },
  children: [{ type: "text", value: "const x = 1;" }],
};

const lang = extractLang(codeElement);

console.log(lang);
// typescript
```

## Ví dụ

### Trích xuất ngôn ngữ từ phần tử code

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-js", "highlight"] },
  children: [{ type: "text", value: "console.log('hello');" }],
};

console.log(extractLang(code));
// js
```

### Khi không có class ngôn ngữ

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["highlight"] },
  children: [{ type: "text", value: "plain text" }],
};

console.log(extractLang(code));
// null
```

### Khi className không phải là mảng

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: {},
  children: [{ type: "text", value: "no classes" }],
};

console.log(extractLang(code));
// null
```

### Kết hợp với findCodeChild

```ts
import { extractLang, findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python"] },
      children: [{ type: "text", value: "print('hello')" }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // python
}
```
