---
title: "figure()"
description: "Envuelve las imágenes que tienen texto alt en elementos <figure> y <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Firma

```ts
function figure(): UnifastPlugin
```

## Parámetros

Ninguno.

## Uso

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Ejemplos

### Envolvido básico en figure

Cuando una imagen tiene texto alt, `figure()` la envuelve en un elemento `<figure>` y añade un `<figcaption>` que contiene el texto alt:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Imagen sin texto alt

Las imágenes sin texto alt no se envuelven, ya que no hay una leyenda significativa que mostrar:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
