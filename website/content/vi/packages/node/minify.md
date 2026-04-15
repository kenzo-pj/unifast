---
title: "minify()"
description: "Minify đầu ra HTML bằng cách loại bỏ khoảng trắng không cần thiết."
---

```ts
import { minify } from "@unifast/node";
```

## Chữ ký

```ts
function minify(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Ví dụ

### Minify cơ bản

Plugin `minify()` gộp các ký tự khoảng trắng liên tiếp thành một dấu cách duy nhất, loại bỏ comment HTML, loại bỏ các nút văn bản chỉ chứa khoảng trắng giữa các phần tử block, và loại bỏ các thuộc tính `class` và `style` rỗng:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Nội dung preformatted được giữ nguyên

Khoảng trắng bên trong các khối `<pre>` và `<code>` được giữ nguyên, nên định dạng code không bao giờ bị phá vỡ:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
