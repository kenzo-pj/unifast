---
title: "imgLazyLoading()"
description: 'Ajoute l''attribut loading="lazy" aux images pour un chargement différé.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Signature

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Paramètres

### options?

Configuration du comportement de chargement différé

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Nombre d'images à ignorer (par exemple, pour exclure l'image de couverture) |

Le plugin ajoute à la fois les attributs `loading="lazy"` et `decoding="async"` aux éléments `<img>` ciblés, y compris aux images imbriquées dans d'autres éléments.

## Utilisation

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Photo 1](photo1.jpg)

![Photo 2](photo2.jpg)

![Photo 3](photo3.jpg)
`;

const result = compile(md, {
  plugins: [imgLazyLoading()],
});

// All images get loading="lazy" and decoding="async":
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## Exemples

### Ignorer la première image (motif d'image de couverture)

La première image d'une page est souvent une image de couverture ou bannière qui doit se charger immédiatement. Utilisez `skipFirst` pour l'exclure du chargement différé :

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Hero banner](hero.jpg)

Some introductory content...

![Diagram](diagram.jpg)

More content...

![Screenshot](screenshot.jpg)
`;

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 1,
    }),
  ],
});

// First image loads eagerly (no loading attribute):
// <img src="hero.jpg" alt="Hero banner">
//
// Remaining images are lazy loaded:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### Ignorer plusieurs images au-dessus de la ligne de flottaison

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // skip the first 3 images
    }),
  ],
});
```
