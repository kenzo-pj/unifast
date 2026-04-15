---
title: "addClasses()"
description: "Aggiunge classi CSS agli elementi che corrispondono a selettori CSS."
---

```ts
import { addClasses } from "@unifast/node";
```

## Firma

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parametri

### rules

Un `Record<string, string>` dove le chiavi sono selettori CSS e i valori sono nomi di classi separati da spazi da aggiungere agli elementi corrispondenti. Le classi vengono unite a quelle eventualmente già presenti sull'elemento.

### Selettori supportati

Supporta un ampio sottoinsieme di CSS Selectors Level 4, incluso:

- **Selettori di tag**: `h1`, `p`, `table`
- **Selettori di classe**: `.info`, `.alert.warning`
- **Selettori di ID**: `#main`
- **Selettore universale**: `*`
- **Selettori di attributo**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinatori**: discendente (` `), figlio (`>`), fratello adiacente (`+`), fratello generale (`~`)
- **Pseudo-classi**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Selettori separati da virgola**: `h1, h2, h3`
- **Selettori composti**: `div.alert#main[data-type="warning"]`

## Utilizzo

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

## Esempi

### Selettori separati da virgola

Applica le stesse classi a più tipi di elemento:

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

### Selettori complessi

Utilizza combinatori e pseudo-classi per un targeting preciso:

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

### Classi utility di Tailwind CSS

Un pattern comune consiste nell'usare `addClasses` per applicare le utility di Tailwind all'HTML generato dal Markdown:

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

### Selettori di attributo

Seleziona gli elementi in base ai loro attributi:

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
