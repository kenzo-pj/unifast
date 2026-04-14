---
title: "addClasses()"
description: "CSS セレクタにマッチする要素へ CSS クラスを追加します。"
---

```ts
import { addClasses } from "@unifast/node";
```

## シグネチャ

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## パラメータ

### rules

`Record<string, string>` 型のオブジェクトで、キーは CSS セレクタ、値はマッチした要素に追加するスペース区切りのクラス名です。クラスは要素の既存のクラスにマージされます。

### サポートするセレクタ

CSS Selectors Level 4 の広範なサブセットをサポートしています。

- **タグセレクタ**: `h1`、`p`、`table`
- **クラスセレクタ**: `.info`、`.alert.warning`
- **ID セレクタ**: `#main`
- **全称セレクタ**: `*`
- **属性セレクタ**: `[data-type]`、`[href^="https"]`、`[href$=".pdf"]`、`[href*="example"]`、`[class~="bar"]`、`[lang|="en"]`
- **結合子**: 子孫 (` `)、子 (`>`)、隣接兄弟 (`+`)、一般兄弟 (`~`)
- **擬似クラス**: `:first-child`、`:last-child`、`:nth-child()`、`:not()`、`:empty`
- **カンマ区切りのセレクタ**: `h1, h2, h3`
- **複合セレクタ**: `div.alert#main[data-type="warning"]`

## 使い方

```ts
import { compile, addClasses } from "@unifast/node";

const md = `
# Hello World

Some paragraph text.

| Name | Value |
|------|-------|
| A    | 1     |
`;

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-3xl font-bold",
      p: "leading-relaxed",
      table: "border-collapse w-full",
    }),
  ],
});

// <h1 class="text-3xl font-bold">Hello World</h1>
// <p class="leading-relaxed">Some paragraph text.</p>
// <table class="border-collapse w-full">...</table>
```

## 使用例

### カンマ区切りのセレクタ

複数の要素種別に同じクラスを適用します。

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "h1, h2, h3": "font-bold tracking-tight",
    }),
  ],
});
```

### 複雑なセレクタ

結合子や擬似クラスを使って対象を正確に指定します。

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "pre > code": "block overflow-x-auto",
      "ul > li:first-child": "mt-0",
      "ul > li:last-child": "mb-0",
      "a[href^=\"https\"]": "external-link",
      "div:not(.alert)": "default-container",
    }),
  ],
});
```

### Tailwind CSS のユーティリティクラス

Markdown から生成された HTML に Tailwind のユーティリティクラスを適用するのは、`addClasses` のよくある使い方です。

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-4xl font-extrabold text-gray-900 mb-8",
      h2: "text-2xl font-bold text-gray-800 mt-12 mb-4",
      h3: "text-xl font-semibold text-gray-700 mt-8 mb-3",
      p: "text-base leading-7 text-gray-600 mb-4",
      a: "text-blue-600 underline hover:text-blue-800",
      blockquote: "border-l-4 border-gray-300 pl-4 italic text-gray-500",
      table: "min-w-full divide-y divide-gray-200",
      "thead th": "px-4 py-2 text-left text-sm font-semibold text-gray-900",
      "tbody td": "px-4 py-2 text-sm text-gray-700",
      "tbody tr:nth-child(2n)": "bg-gray-50",
      img: "rounded-lg shadow-md",
      pre: "rounded-lg overflow-hidden",
      "pre > code": "block p-4 text-sm",
    }),
  ],
});
```

### 属性セレクタ

属性に基づいて要素を対象にします。

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "[href$=\".pdf\"]": "pdf-link",
      "[href^=\"https\"]": "external",
      "[data-type]": "has-type",
    }),
  ],
});
```
