---
title: "addClasses()"
description: "Ajoute des classes CSS aux éléments correspondant à des sélecteurs CSS."
---

```ts
import { addClasses } from "@unifast/node";
```

## Signature

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Paramètres

### rules

Un `Record<string, string>` dont les clés sont des sélecteurs CSS et les valeurs des noms de classes séparés par des espaces, à ajouter aux éléments correspondants. Les classes sont fusionnées avec toutes les classes déjà présentes sur l'élément.

### Sélecteurs pris en charge

Prend en charge un large sous-ensemble de CSS Selectors Level 4, notamment :

- **Sélecteurs de balise** : `h1`, `p`, `table`
- **Sélecteurs de classe** : `.info`, `.alert.warning`
- **Sélecteurs d'ID** : `#main`
- **Sélecteur universel** : `*`
- **Sélecteurs d'attribut** : `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Combinateurs** : descendant (` `), enfant (`>`), frère adjacent (`+`), frère général (`~`)
- **Pseudo-classes** : `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Sélecteurs séparés par des virgules** : `h1, h2, h3`
- **Sélecteurs composés** : `div.alert#main[data-type="warning"]`

## Utilisation

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

## Exemples

### Sélecteurs séparés par des virgules

Appliquez les mêmes classes à plusieurs types d'éléments :

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

### Sélecteurs avancés

Utilisez les combinateurs et les pseudo-classes pour un ciblage précis :

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

### Classes utilitaires Tailwind CSS

Une utilisation courante consiste à employer `addClasses` pour appliquer des utilitaires Tailwind au HTML généré depuis Markdown :

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

### Sélecteurs d'attribut

Ciblez les éléments en fonction de leurs attributs :

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
