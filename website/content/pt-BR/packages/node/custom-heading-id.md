---
title: "customHeadingId()"
description: "Define IDs customizados em headings usando a sintaxe {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Assinatura

```ts
function customHeadingId(): UnifastPlugin
```

## Parâmetros

Nenhum.

## Uso

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Exemplos

### ID customizado

Use a sintaxe `{#custom-id}` no final de um heading para atribuir um atributo `id` específico. O bloco entre chaves é removido do texto renderizado:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Heading sem ID customizado

Headings sem a sintaxe `{#...}` são deixados inalterados. Eles usam o slug padrão gerado a partir do texto do heading:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Classes e atributos arbitrários

A sintaxe entre chaves também suporta as notações `.class` e `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
