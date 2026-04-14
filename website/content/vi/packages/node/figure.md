---
title: "figure()"
description: "Bọc các hình ảnh có alt text trong phần tử <figure> và <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Chữ ký

```ts
function figure(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Ví dụ

### Bọc figure cơ bản

Khi một hình ảnh có alt text, `figure()` sẽ bọc nó trong phần tử `<figure>` và thêm một `<figcaption>` chứa alt text:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Hình ảnh không có alt text

Các hình ảnh không có alt text sẽ không được bọc, vì không có caption có ý nghĩa để hiển thị:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
