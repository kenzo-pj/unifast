---
title: "figure()"
description: "將帶有 alt 文字的圖片包進 <figure> 與 <figcaption> 元素中。"
---

```ts
import { figure } from "@unifast/node";
```

## 函式簽名

```ts
function figure(): UnifastPlugin
```

## 參數

無。

## 用法

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## 範例

### 基本的 figure 包裹

當圖片具備 alt 文字時，`figure()` 會將其包進 `<figure>` 元素中，並加入一個內容為 alt 文字的 `<figcaption>`：

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### 沒有 alt 文字的圖片

未提供 alt 文字的圖片不會被包裹，因為沒有具意義的說明可供顯示：

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
