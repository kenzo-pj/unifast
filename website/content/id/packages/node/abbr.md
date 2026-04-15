---
title: "abbr()"
description: "Mengonversi definisi singkatan menjadi elemen <abbr> dengan atribut title."
---

```ts
import { abbr } from "@unifast/node";
```

## Signature

```ts
function abbr(): UnifastPlugin
```

## Parameter

Tidak ada.

## Penggunaan

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// Occurrences of "HTML" are wrapped in <abbr title="Hyper Text Markup Language">
```

## Contoh

### Singkatan dasar

Definisikan sebuah singkatan dengan sintaks `*[TERM]: Definition`. Paragraf definisi dihapus dari output, dan semua kemunculan istilah tersebut dibungkus dalam elemen `<abbr>`:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Beberapa singkatan

Anda dapat mendefinisikan beberapa singkatan. Setiap istilah diganti secara independen di seluruh dokumen:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
