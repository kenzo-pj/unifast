---
title: "abbr()"
description: "Convertit les définitions d'abréviations en éléments <abbr> dotés d'attributs title."
---

```ts
import { abbr } from "@unifast/node";
```

## Signature

```ts
function abbr(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Exemples

### Abréviation de base

Définissez une abréviation avec la syntaxe `*[TERME]: Définition`. Le paragraphe de définition est retiré de la sortie, et toutes les occurrences du terme sont enveloppées dans des éléments `<abbr>` :

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Plusieurs abréviations

Vous pouvez définir plusieurs abréviations. Chaque terme est remplacé indépendamment dans l'ensemble du document :

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
