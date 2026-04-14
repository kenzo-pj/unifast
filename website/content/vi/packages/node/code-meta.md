---
title: "codeMeta()"
description: "Parse chuỗi meta của code fence thành các thuộc tính data trên khối mã."
---

```ts
import { codeMeta } from "@unifast/node";
```

## Chữ ký

```ts
function codeMeta(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// The <pre> element gets data-title="example.js"
```

## Ví dụ

### Parse meta cơ bản

Plugin `codeMeta()` parse chuỗi meta đứng sau định danh ngôn ngữ trong các khối mã fenced và chuyển các key được nhận diện thành thuộc tính `data-*` trên phần tử `<pre>`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="app.ts"
const x = 1;
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
console.log(result.output);
// <pre data-lang="js" data-title="app.ts"><code class="language-js">const x = 1;
// </code></pre>
```

### Nhiều thuộc tính meta

Bạn có thể kết hợp nhiều thuộc tính meta như `title`, `{1,3-5}` để highlight dòng, `showLineNumbers`, `diff` và `wordWrap`:

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// The <pre> element receives:
//   data-title="server.ts"
//   Lines 1 and 3 get data-highlighted attributes
//   showLineNumbers is recognized as a boolean flag
```
