---
title: "addClasses()"
description: "Thêm class CSS vào các phần tử khớp với CSS selector."
---

```ts
import { addClasses } from "@unifast/node";
```

## Chữ ký

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Tham số

### rules

Một `Record<string, string>` trong đó key là CSS selector và value là các tên class (cách nhau bằng dấu cách) để thêm vào các phần tử khớp. Các class sẽ được gộp cùng các class đã có sẵn trên phần tử.

### Các selector được hỗ trợ

Hỗ trợ một tập con lớn của CSS Selectors Level 4 bao gồm:

- **Selector theo thẻ**: `h1`, `p`, `table`
- **Selector theo class**: `.info`, `.alert.warning`
- **Selector theo ID**: `#main`
- **Selector universal**: `*`
- **Selector theo thuộc tính**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinator**: descendant (` `), child (`>`), adjacent sibling (`+`), general sibling (`~`)
- **Pseudo-class**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Selector phân tách bằng dấu phẩy**: `h1, h2, h3`
- **Compound selector**: `div.alert#main[data-type="warning"]`

## Cách dùng

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

## Ví dụ

### Selector phân tách bằng dấu phẩy

Áp dụng cùng một tập class cho nhiều loại phần tử:

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

### Selector phức tạp

Sử dụng combinator và pseudo-class để nhắm mục tiêu chính xác:

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

### Class tiện ích của Tailwind CSS

Một pattern phổ biến là dùng `addClasses` để áp dụng các utility của Tailwind lên HTML được sinh ra từ Markdown:

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

### Selector theo thuộc tính

Nhắm vào các phần tử dựa trên thuộc tính của chúng:

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
