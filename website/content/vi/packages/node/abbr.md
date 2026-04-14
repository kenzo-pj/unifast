---
title: "abbr()"
description: "Chuyển các định nghĩa viết tắt thành phần tử <abbr> kèm thuộc tính title."
---

```ts
import { abbr } from "@unifast/node";
```

## Chữ ký

```ts
function abbr(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Ví dụ

### Viết tắt cơ bản

Định nghĩa một chữ viết tắt bằng cú pháp `*[TERM]: Definition`. Đoạn định nghĩa sẽ bị loại bỏ khỏi đầu ra, và tất cả các lần xuất hiện của từ viết tắt được bọc trong phần tử `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Nhiều chữ viết tắt

Bạn có thể định nghĩa nhiều chữ viết tắt. Mỗi từ sẽ được thay thế độc lập trong toàn bộ tài liệu:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
