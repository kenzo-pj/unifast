---
title: "abbr()"
description: "Converte le definizioni di abbreviazioni in elementi <abbr> con attributi title."
---

```ts
import { abbr } from "@unifast/node";
```

## Firma

```ts
function abbr(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Esempi

### Abbreviazione di base

Definisci un'abbreviazione con la sintassi `*[TERMINE]: Definizione`. Il paragrafo di definizione viene rimosso dall'output e tutte le occorrenze del termine vengono racchiuse in elementi `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Più abbreviazioni

È possibile definire più abbreviazioni. Ogni termine viene sostituito in modo indipendente in tutto il documento:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
