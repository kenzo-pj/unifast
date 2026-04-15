---
title: "readingTime()"
description: "Memperkirakan waktu baca dokumen dan menyertakannya pada hasil compile."
---

```ts
import { readingTime } from "@unifast/node";
```

## Signature

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfigurasi untuk estimasi waktu baca

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Kata per menit untuk teks Latin |
| `cjkCharsPerMinute` | `number` | `500` | Karakter per menit untuk teks CJK |

## Return

Plugin menambahkan properti `readingTime` ke hasil compile:

| Properti | Tipe | Deskripsi |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Estimasi waktu baca dalam menit (minimum 1, dibulatkan ke atas ke kelipatan 0.5 terdekat) |
| `result.readingTime.words` | `number` | Total jumlah kata (kata Latin + karakter CJK) |

## Penggunaan

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# My Article

This is a short article with some content that demonstrates
reading time estimation.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

console.log(result.readingTime);
// { minutes: 1, words: 16 }
```

## Contoh

### Kata per menit kustom

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // slower reading speed
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### Konten CJK

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // adjust for CJK reading speed
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Teks campuran Latin dan CJK

Waktu baca dihitung secara terpisah untuk kata Latin dan karakter CJK, kemudian digabungkan. Code block dikecualikan dari hitungan kata.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Latin words counted at 200 WPM, CJK characters at 500 CPM
console.log(result.readingTime);
```
