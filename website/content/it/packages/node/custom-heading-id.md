---
title: "customHeadingId()"
description: "Imposta ID personalizzati sui titoli utilizzando la sintassi {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Firma

```ts
function customHeadingId(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Esempi

### ID personalizzato

Usa la sintassi `{#custom-id}` alla fine di un titolo per assegnare un attributo `id` specifico. Il blocco tra parentesi graffe viene rimosso dal testo renderizzato:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Titolo senza ID personalizzato

I titoli privi della sintassi `{#...}` restano invariati. Utilizzano lo slug predefinito generato a partire dal testo del titolo:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Classi e attributi arbitrari

La sintassi con parentesi graffe supporta anche la notazione `.class` e `key=value`:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
