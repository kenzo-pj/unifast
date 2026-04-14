---
title: "imgLazyLoading()"
description: 'Añade el atributo loading="lazy" a las imágenes para carga diferida.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Firma

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parámetros

### options?

Configuración del comportamiento de lazy loading

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Número de imágenes a omitir (p. ej., para omitir la imagen hero) |

El plugin añade tanto `loading="lazy"` como `decoding="async"` a los elementos `<img>` coincidentes, incluyendo las imágenes anidadas dentro de otros elementos.

## Uso

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

## Ejemplos

### Omitir la primera imagen (patrón de imagen hero)

La primera imagen de una página suele ser una imagen hero o de banner que debe cargarse inmediatamente. Usa `skipFirst` para excluirla del lazy loading:

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

### Omitir varias imágenes above-the-fold

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
