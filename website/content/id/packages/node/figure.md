---
title: "figure()"
description: "Membungkus gambar yang memiliki alt text dalam elemen <figure> dan <figcaption>."
---

```ts
import { figure } from "@unifast/node";
```

## Signature

```ts
function figure(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Contoh

### Pembungkusan figure dasar

Ketika sebuah gambar memiliki alt text, `figure()` membungkusnya dalam elemen `<figure>` dan menambahkan `<figcaption>` yang berisi alt text tersebut:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Gambar tanpa alt text

Gambar tanpa alt text tidak dibungkus, karena tidak ada caption yang bermakna untuk ditampilkan:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
