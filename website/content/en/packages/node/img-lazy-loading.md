---
title: "imgLazyLoading()"
description: 'Add loading="lazy" attribute to images for deferred loading.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Signature

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parameters

### options?

Configuration for lazy loading behavior

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Number of images to skip (e.g. skip hero image) |

The plugin adds both `loading="lazy"` and `decoding="async"` attributes to matched `<img>` elements, including images nested inside other elements.

## Usage

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

## Examples

### Skip first image (hero image pattern)

The first image on a page is often a hero or banner image that should load eagerly. Use `skipFirst` to exclude it from lazy loading:

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

### Skip multiple above-the-fold images

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
