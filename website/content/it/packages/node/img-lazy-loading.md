---
title: "imgLazyLoading()"
description: 'Aggiunge l''attributo loading="lazy" alle immagini per il caricamento differito.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Firma

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parametri

### options?

Configurazione del comportamento del lazy loading

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `skipFirst` | `number` | `0` | Numero di immagini da saltare (ad esempio per saltare l'immagine hero) |

Il plugin aggiunge sia l'attributo `loading="lazy"` che `decoding="async"` agli elementi `<img>` corrispondenti, incluse le immagini annidate all'interno di altri elementi.

## Utilizzo

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

## Esempi

### Saltare la prima immagine (pattern hero image)

La prima immagine di una pagina è spesso un'immagine hero o banner che dovrebbe essere caricata immediatamente. Usa `skipFirst` per escluderla dal lazy loading:

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

### Saltare più immagini above-the-fold

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
