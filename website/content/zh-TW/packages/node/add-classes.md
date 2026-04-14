---
title: "addClasses()"
description: "為符合 CSS 選擇器的元素加上 CSS class。"
---

```ts
import { addClasses } from "@unifast/node";
```

## 函式簽名

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## 參數

### rules

一個 `Record<string, string>`，鍵為 CSS 選擇器，值為要加到符合元素上、以空白分隔的 class 名稱。新增的 class 會與元素現有的 class 合併。

### 支援的選擇器

支援廣泛的 CSS Selectors Level 4 子集，包含：

- **標籤選擇器**：`h1`、`p`、`table`
- **Class 選擇器**：`.info`、`.alert.warning`
- **ID 選擇器**：`#main`
- **通用選擇器**：`*`
- **屬性選擇器**：`[data-type]`、`[href^="https"]`、`[href$=".pdf"]`、`[href*="example"]`、`[class~="bar"]`、`[lang|="en"]`
- **組合器**：後代（` `）、子代（`>`）、相鄰兄弟（`+`）、一般兄弟（`~`）
- **偽類別**：`:first-child`、`:last-child`、`:nth-child()`、`:not()`、`:empty`
- **以逗號分隔的選擇器**：`h1, h2, h3`
- **複合選擇器**：`div.alert#main[data-type="warning"]`

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

## 範例

### 以逗號分隔的選擇器

將相同的 class 套用至多種元素類型：

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

### 複雜的選擇器

使用組合器與偽類別進行精確的指定：

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

### Tailwind CSS 工具 class

常見的做法是使用 `addClasses` 將 Tailwind 工具 class 套用到 Markdown 產生的 HTML 上：

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

### 屬性選擇器

根據屬性鎖定元素：

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
