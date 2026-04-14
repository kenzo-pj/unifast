---
title: "addClasses()"
description: "Menambahkan class CSS ke elemen yang cocok dengan selector CSS."
---

```ts
import { addClasses } from "@unifast/node";
```

## Signature

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parameter

### rules

Sebuah `Record<string, string>` dengan key berupa selector CSS dan value berupa nama class yang dipisahkan spasi untuk ditambahkan ke elemen yang cocok. Class akan digabungkan dengan class yang sudah ada pada elemen.

### Selector yang didukung

Mendukung subset luas dari CSS Selectors Level 4, termasuk:

- **Tag selector**: `h1`, `p`, `table`
- **Class selector**: `.info`, `.alert.warning`
- **ID selector**: `#main`
- **Universal selector**: `*`
- **Attribute selector**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinator**: descendant (` `), child (`>`), adjacent sibling (`+`), general sibling (`~`)
- **Pseudo-class**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Selector dipisahkan koma**: `h1, h2, h3`
- **Compound selector**: `div.alert#main[data-type="warning"]`

## Penggunaan

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

## Contoh

### Selector dipisahkan koma

Terapkan class yang sama ke beberapa tipe elemen:

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

### Selector kompleks

Gunakan combinator dan pseudo-class untuk penargetan yang presisi:

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

### Class utility Tailwind CSS

Pola umum adalah menggunakan `addClasses` untuk menerapkan utility Tailwind ke HTML yang dihasilkan dari Markdown:

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

### Attribute selector

Targetkan elemen berdasarkan atributnya:

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
