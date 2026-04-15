---
title: "figure()"
description: "Racchiude le immagini dotate di testo alt in elementi <figure> e <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Firma

```ts
function figure(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Esempi

### Incapsulamento di base in figure

Quando un'immagine ha un testo alt, `figure()` la racchiude in un elemento `<figure>` e aggiunge una `<figcaption>` contenente il testo alt:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Immagine senza testo alt

Le immagini senza testo alt non vengono incapsulate, poiché non vi è alcuna didascalia significativa da mostrare:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
