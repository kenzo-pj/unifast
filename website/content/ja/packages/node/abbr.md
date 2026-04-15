---
title: "abbr()"
description: "略語の定義を、title 属性付きの <abbr> 要素へ変換します。"
---

```ts
import { abbr } from "@unifast/node";
```

## シグネチャ

```ts
function abbr(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## 使用例

### 基本的な略語

`*[TERM]: Definition` という構文で略語を定義します。定義の段落は出力から除去され、文書内に現れる用語は `<abbr>` 要素でラップされます。

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### 複数の略語

複数の略語を定義できます。それぞれの用語は文書全体で独立して置き換えられます。

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
