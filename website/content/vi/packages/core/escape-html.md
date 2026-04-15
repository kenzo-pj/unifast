---
title: "escapeHtml()"
description: "Escape các ký tự đặc biệt của HTML trong một chuỗi."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Chữ ký

```ts
function escapeHtml(str: string): string
```

## Tham số

### str

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `str` | `string` | — | Chuỗi chứa các ký tự cần escape |

## Giá trị trả về

`string` — Chuỗi đầu vào với các ký tự `&`, `<`, `>` và `"` được thay bằng các HTML entity tương ứng.

## Cách dùng

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Ví dụ

### Escape cơ bản

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Escape nội dung do người dùng tạo

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Xây dựng thuộc tính HTML an toàn

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Hành vi

- **`&`** được thay bằng `&amp;`
- **`<`** được thay bằng `&lt;`
- **`>`** được thay bằng `&gt;`
- **`"`** được thay bằng `&quot;`
- Tất cả các ký tự khác giữ nguyên
