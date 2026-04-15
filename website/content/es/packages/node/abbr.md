---
title: "abbr()"
description: "Convierte las definiciones de abreviaciones en elementos <abbr> con atributos title."
---

```ts
import { abbr } from "@unifast/node";
```

## Firma

```ts
function abbr(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Ejemplos

### Abreviación básica

Define una abreviación con la sintaxis `*[TERM]: Definition`. El párrafo de definición se elimina de la salida, y todas las apariciones del término se envuelven en elementos `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Múltiples abreviaciones

Puedes definir múltiples abreviaciones. Cada término se reemplaza de forma independiente a lo largo del documento:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
