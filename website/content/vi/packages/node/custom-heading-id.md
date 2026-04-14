---
title: "customHeadingId()"
description: "Đặt ID tùy chỉnh cho tiêu đề bằng cú pháp {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Chữ ký

```ts
function customHeadingId(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Ví dụ

### ID tùy chỉnh

Sử dụng cú pháp `{#custom-id}` ở cuối một tiêu đề để gán thuộc tính `id` cụ thể. Khối nằm trong dấu ngoặc nhọn sẽ bị loại bỏ khỏi văn bản hiển thị:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Tiêu đề không có ID tùy chỉnh

Các tiêu đề không có cú pháp `{#...}` được giữ nguyên. Chúng sử dụng slug mặc định được sinh từ nội dung tiêu đề:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Class và các thuộc tính tùy ý

Cú pháp dấu ngoặc nhọn cũng hỗ trợ ký hiệu `.class` và `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
