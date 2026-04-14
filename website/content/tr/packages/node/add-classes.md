---
title: "addClasses()"
description: "CSS seçicileriyle eşleşen elemanlara CSS sınıfları ekler."
---

```ts
import { addClasses } from "@unifast/node";
```

## İmza

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## Parametreler

### rules

Anahtarların CSS seçicileri olduğu ve değerlerin eşleşen elemanlara eklenecek boşlukla ayrılmış sınıf adları olduğu bir `Record<string, string>`. Sınıflar, elemandaki mevcut sınıflarla birleştirilir.

### Desteklenen seçiciler

Aşağıdakiler dahil CSS Selectors Level 4'ün geniş bir alt kümesini destekler:

- **Etiket seçicileri**: `h1`, `p`, `table`
- **Sınıf seçicileri**: `.info`, `.alert.warning`
- **ID seçicileri**: `#main`
- **Evrensel seçici**: `*`
- **Öznitelik seçicileri**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **Birleştiriciler**: torun (` `), çocuk (`>`), bitişik kardeş (`+`), genel kardeş (`~`)
- **Yalancı sınıflar**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **Virgülle ayrılmış seçiciler**: `h1, h2, h3`
- **Bileşik seçiciler**: `div.alert#main[data-type="warning"]`

## Kullanım

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

## Örnekler

### Virgülle ayrılmış seçiciler

Aynı sınıfları birden fazla eleman türüne uygulayın:

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

### Karmaşık seçiciler

Kesin hedefleme için birleştiricileri ve yalancı sınıfları kullanın:

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

### Tailwind CSS yardımcı sınıfları

Yaygın bir kalıp, Markdown ile üretilen HTML'e Tailwind yardımcı sınıflarını uygulamak için `addClasses` kullanmaktır:

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

### Öznitelik seçicileri

Öznitelikleri temel alarak elemanları hedefleyin:

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
