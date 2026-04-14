---
title: "addClasses()"
description: "Adiciona classes CSS a elementos que correspondem a seletores CSS."
---

```ts
import { addClasses } from "@unifast/node";
```

## Assinatura

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parâmetros

### rules

Um `Record<string, string>` em que as chaves são seletores CSS e os valores são nomes de classes separados por espaços a serem adicionados aos elementos correspondentes. As classes são mescladas com quaisquer classes existentes no elemento.

### Seletores suportados

Suporta um subconjunto amplo do CSS Selectors Level 4, incluindo:

- **Seletores de tag**: `h1`, `p`, `table`
- **Seletores de classe**: `.info`, `.alert.warning`
- **Seletores de ID**: `#main`
- **Seletor universal**: `*`
- **Seletores de atributo**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinadores**: descendente (` `), filho (`>`), irmão adjacente (`+`), irmão geral (`~`)
- **Pseudo-classes**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Seletores separados por vírgula**: `h1, h2, h3`
- **Seletores compostos**: `div.alert#main[data-type="warning"]`

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

## Exemplos

### Seletores separados por vírgula

Aplique as mesmas classes a múltiplos tipos de elemento:

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

### Seletores complexos

Use combinadores e pseudo-classes para uma seleção precisa:

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

### Classes utilitárias do Tailwind CSS

Um padrão comum é usar `addClasses` para aplicar utilitários do Tailwind ao HTML gerado por Markdown:

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

### Seletores de atributo

Selecione elementos com base em seus atributos:

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
