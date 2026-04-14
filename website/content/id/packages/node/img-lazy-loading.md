---
title: "imgLazyLoading()"
description: 'Menambahkan atribut loading="lazy" pada gambar untuk pemuatan tertunda.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Signature

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfigurasi untuk perilaku lazy loading

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Jumlah gambar yang dilewati (misalnya melewati gambar hero) |

Plugin menambahkan atribut `loading="lazy"` dan `decoding="async"` pada elemen `<img>` yang cocok, termasuk gambar yang bersarang di dalam elemen lain.

## Penggunaan

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

## Contoh

### Melewati gambar pertama (pola hero image)

Gambar pertama pada sebuah halaman seringkali merupakan gambar hero atau banner yang harus dimuat secara eager. Gunakan `skipFirst` untuk mengecualikannya dari lazy loading:

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

### Melewati beberapa gambar above-the-fold

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
