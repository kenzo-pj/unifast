---
title: "abbr()"
description: "将缩写定义转换为带有 title 属性的 <abbr> 元素。"
---

```ts
import { abbr } from "@unifast/node";
```

## 签名

```ts
function abbr(): UnifastPlugin
```

## 参数

无。

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

## 示例

### 基础缩写

使用 `*[TERM]: Definition` 语法定义一个缩写。定义所在的段落会从输出中移除，文档中所有对该词的引用都会被包裹在 `<abbr>` 元素中：

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### 多个缩写

你可以同时定义多个缩写。每个术语会在整篇文档中独立替换：

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
