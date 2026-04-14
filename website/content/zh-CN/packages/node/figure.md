---
title: "figure()"
description: "将带有 alt 文本的图片包裹在 <figure> 和 <figcaption> 元素中。"
---

```ts
import { figure } from "@unifast/node";
```

## 签名

```ts
function figure(): UnifastPlugin
```

## 参数

无。

## 用法

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## 示例

### 基础 figure 包裹

当图片带有 alt 文本时，`figure()` 会把它包裹在 `<figure>` 元素中，并添加一个包含 alt 文本的 `<figcaption>`：

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### 未提供 alt 文本的图片

未提供 alt 文本的图片不会被包裹，因为此时没有有意义的说明文字可以展示：

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
