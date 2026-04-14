---
title: "addClasses()"
description: "为匹配 CSS 选择器的元素添加 CSS 类名。"
---

```ts
import { addClasses } from "@unifast/node";
```

## 签名

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## 参数

### rules

一个 `Record<string, string>`，其中键为 CSS 选择器，值为用空格分隔的类名，将被添加到匹配的元素上。新增的类名会与元素上已有的类名合并。

### 支持的选择器

支持 CSS Selectors Level 4 的大部分子集，包括：

- **标签选择器**：`h1`、`p`、`table`
- **类选择器**：`.info`、`.alert.warning`
- **ID 选择器**：`#main`
- **通用选择器**：`*`
- **属性选择器**：`[data-type]`、`[href^="https"]`、`[href$=".pdf"]`、`[href*="example"]`、`[class~="bar"]`、`[lang|="en"]`
- **组合器**：后代（` `）、子元素（`>`）、相邻兄弟（`+`）、通用兄弟（`~`）
- **伪类**：`:first-child`、`:last-child`、`:nth-child()`、`:not()`、`:empty`
- **逗号分隔的选择器**：`h1, h2, h3`
- **复合选择器**：`div.alert#main[data-type="warning"]`

## 用法

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

## 示例

### 逗号分隔的选择器

把相同的类名一次性应用到多种元素上：

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

### 复杂选择器

使用组合器和伪类实现精确的目标选择：

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

### Tailwind CSS 工具类

一种常见的用法是使用 `addClasses` 为 Markdown 生成的 HTML 应用 Tailwind 工具类：

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

### 属性选择器

根据属性来选中元素：

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
