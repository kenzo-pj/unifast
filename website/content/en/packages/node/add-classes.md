---
title: "addClasses()"
description: "Add CSS classes to elements matching CSS selectors."
---

```ts
import { addClasses } from "@unifast/node";
```

## Signature

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parameters

### rules

A `Record<string, string>` where keys are CSS selectors and values are space-separated class names to add to matching elements. Classes are merged with any existing classes on the element.

### Supported selectors

Supports full CSS Selectors Level 4 including:

- **Tag selectors**: `h1`, `p`, `table`
- **Class selectors**: `.info`, `.alert.warning`
- **ID selectors**: `#main`
- **Universal selector**: `*`
- **Attribute selectors**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinators**: descendant (` `), child (`>`), adjacent sibling (`+`), general sibling (`~`)
- **Pseudo-classes**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Comma-separated selectors**: `h1, h2, h3`
- **Compound selectors**: `div.alert#main[data-type="warning"]`

## Usage

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

## Examples

### Comma-separated selectors

Apply the same classes to multiple element types:

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

### Complex selectors

Use combinators and pseudo-classes for precise targeting:

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

### Tailwind CSS utility classes

A common pattern is using `addClasses` to apply Tailwind utilities to Markdown-generated HTML:

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

### Attribute selectors

Target elements based on their attributes:

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
