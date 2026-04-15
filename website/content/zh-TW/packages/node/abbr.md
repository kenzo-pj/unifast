---
title: "abbr()"
description: "將縮寫定義轉換為帶有 title 屬性的 <abbr> 元素。"
---

```ts
import { abbr } from "@unifast/node";
```

## 函式簽名

```ts
function abbr(): UnifastPlugin
```

## 參數

無。

## 用法

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## 範例

### 基本縮寫

以 `*[TERM]: Definition` 語法定義縮寫。定義段落會從輸出中移除，所有出現該術語的地方都會被包在 `<abbr>` 元素中：

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### 多個縮寫

您可以定義多個縮寫。每個術語會在整份文件中被獨立替換：

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
