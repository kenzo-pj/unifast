---
title: "excerpt()"
description: "Mengekstrak excerpt (kutipan) dari konten dokumen."
---

```ts
import { excerpt } from "@unifast/node";
```

## Signature

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfigurasi untuk ekstraksi excerpt

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Penanda komentar yang memisahkan excerpt dari sisanya |
| `fallbackParagraphs` | `number` | `1` | Jumlah paragraf awal yang digunakan sebagai excerpt ketika separator tidak ditemukan |
| `fallbackCharacters` | `number` | `undefined` | Panjang karakter maksimum untuk excerpt fallback (memotong pada batas kata) |

Ketika `fallbackParagraphs` dan `fallbackCharacters` keduanya diatur, `fallbackParagraphs` lebih diutamakan.

## Return

Plugin menambahkan properti `excerpt` ke hasil compile:

| Properti | Tipe | Deskripsi |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Excerpt teks polos yang diekstrak dari dokumen |

## Penggunaan

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Contoh

### Dengan penanda separator

Tempatkan komentar `<!-- more -->` di Markdown Anda untuk secara eksplisit menandai di mana excerpt berakhir:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### Fallback ke paragraf pertama

Ketika tidak ada penanda separator yang ditemukan, plugin akan fallback dengan mengekstrak N paragraf pertama:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Fallback ke batas karakter

Memotong excerpt ke panjang karakter maksimum, memutus pada batas kata:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
