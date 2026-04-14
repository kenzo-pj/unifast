---
title: "commentRemoval()"
description: "Loại bỏ các comment HTML khỏi đầu ra."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Chữ ký

```ts
function commentRemoval(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## Ví dụ

### Loại bỏ comment cơ bản

Tất cả các nút comment HTML (`<!-- ... -->`) đều bị loại bỏ khỏi cây đầu ra, bao gồm cả các comment được lồng bên trong các phần tử block như blockquote:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### HTML không phải comment vẫn được giữ lại

Chỉ các nút comment bị loại bỏ. Các HTML inline khác được giữ nguyên:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
