---
title: "imgLazyLoading()"
description: 'Fügt Bildern das Attribut loading="lazy" für verzögertes Laden hinzu.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Signatur

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfiguration für das Verhalten des Lazy Loadings

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Anzahl der zu überspringenden Bilder (z. B. das Hero-Bild) |

Das Plugin fügt den passenden `<img>`-Elementen sowohl das Attribut `loading="lazy"` als auch `decoding="async"` hinzu, einschließlich der Bilder, die in anderen Elementen verschachtelt sind.

## Verwendung

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

## Beispiele

### Erstes Bild überspringen (Hero-Bild-Muster)

Das erste Bild auf einer Seite ist oft ein Hero- oder Banner-Bild, das eifrig geladen werden sollte. Verwenden Sie `skipFirst`, um es vom Lazy Loading auszuschließen:

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

### Mehrere Above-the-Fold-Bilder überspringen

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
