---
title: "abbr()"
description: "Converte definições de abreviações em elementos <abbr> com atributos title."
---

```ts
import { abbr } from "@unifast/node";
```

## Assinatura

```ts
function abbr(): UnifastPlugin
```

## Parâmetros

Nenhum.

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

## Exemplos

### Abreviação básica

Defina uma abreviação com a sintaxe `*[TERMO]: Definição`. O parágrafo de definição é removido da saída e todas as ocorrências do termo são envolvidas em elementos `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Múltiplas abreviações

Você pode definir múltiplas abreviações. Cada termo é substituído independentemente em todo o documento:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
