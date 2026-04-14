---
title: "figure()"
description: "Envolve imagens que possuem alt text em elementos <figure> e <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Assinatura

```ts
function figure(): UnifastPlugin
```

## Parâmetros

Nenhum.

## Uso

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Exemplos

### Envolvimento básico em figure

Quando uma imagem tem alt text, o `figure()` a envolve em um elemento `<figure>` e adiciona um `<figcaption>` contendo o alt text:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Imagem sem alt text

Imagens sem alt text não são envolvidas, já que não há legenda significativa para exibir:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
