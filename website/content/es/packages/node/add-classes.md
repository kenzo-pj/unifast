---
title: "addClasses()"
description: "Añade clases CSS a los elementos que coinciden con selectores CSS."
---

```ts
import { addClasses } from "@unifast/node";
```

## Firma

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parámetros

### rules

Un `Record<string, string>` donde las claves son selectores CSS y los valores son nombres de clase separados por espacios que se añadirán a los elementos coincidentes. Las clases se combinan con cualquier clase existente en el elemento.

### Selectores soportados

Soporta un amplio subconjunto de CSS Selectors Level 4, incluyendo:

- **Selectores de etiqueta**: `h1`, `p`, `table`
- **Selectores de clase**: `.info`, `.alert.warning`
- **Selectores de ID**: `#main`
- **Selector universal**: `*`
- **Selectores de atributo**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinadores**: descendiente (` `), hijo (`>`), hermano adyacente (`+`), hermano general (`~`)
- **Pseudo-clases**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Selectores separados por coma**: `h1, h2, h3`
- **Selectores compuestos**: `div.alert#main[data-type="warning"]`

## Uso

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

## Ejemplos

### Selectores separados por coma

Aplica las mismas clases a múltiples tipos de elementos:

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

### Selectores complejos

Usa combinadores y pseudo-clases para una selección precisa:

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

### Clases utilitarias de Tailwind CSS

Un patrón común es usar `addClasses` para aplicar utilidades de Tailwind al HTML generado a partir de Markdown:

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

### Selectores de atributo

Apunta a elementos basándote en sus atributos:

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
