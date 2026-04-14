---
title: "figure()"
description: "Enveloppe les images dotées de texte alternatif dans des éléments <figure> et <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Signature

```ts
function figure(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Exemples

### Encapsulation de figure de base

Lorsqu'une image est accompagnée d'un texte alternatif, `figure()` l'enveloppe dans un élément `<figure>` et ajoute une `<figcaption>` contenant ce texte alternatif :

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Image sans texte alternatif

Les images sans texte alternatif ne sont pas encapsulées, car aucune légende pertinente ne pourrait être affichée :

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
