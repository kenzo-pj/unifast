---
title: "abbr()"
description: "Konvertiert Abkürzungsdefinitionen in <abbr>-Elemente mit title-Attributen."
---

```ts
import { abbr } from "@unifast/node";
```

## Signatur

```ts
function abbr(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Beispiele

### Einfache Abkürzung

Definieren Sie eine Abkürzung mit der Syntax `*[TERM]: Definition`. Der Definitionsabsatz wird aus der Ausgabe entfernt, und alle Vorkommen des Begriffs werden in `<abbr>`-Elemente eingeschlossen:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Mehrere Abkürzungen

Sie können mehrere Abkürzungen definieren. Jeder Begriff wird im gesamten Dokument unabhängig ersetzt:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
