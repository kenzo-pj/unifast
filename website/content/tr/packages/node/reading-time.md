---
title: "readingTime()"
description: "Belgenin okuma süresini tahmin eder ve derleme sonucuna dahil eder."
---

```ts
import { readingTime } from "@unifast/node";
```

## İmza

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parametreler

### options?

Okuma süresi tahmini yapılandırması

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `wordsPerMinute` | `number` | `200` | Latin metin için dakika başına kelime |
| `cjkCharsPerMinute` | `number` | `500` | CJK metin için dakika başına karakter |

## Dönüş Değeri

Plugin, derleme sonucuna bir `readingTime` özelliği ekler:

| Özellik | Tür | Açıklama |
|---------|-----|----------|
| `result.readingTime.minutes` | `number` | Dakika cinsinden tahmini okuma süresi (minimum 1, en yakın 0,5'e yuvarlanır) |
| `result.readingTime.words` | `number` | Toplam kelime sayısı (Latin kelimeler + CJK karakterler) |

## Kullanım

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

## Örnekler

### Özel dakika başına kelime

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // daha yavaş okuma hızı
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### CJK içerik

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // CJK okuma hızı için ayarla
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Karışık Latin ve CJK metin

Okuma süresi, Latin kelimeler ve CJK karakterler için ayrı ayrı hesaplanır, ardından birleştirilir. Kod blokları kelime sayımından hariç tutulur.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Latin kelimeler 200 WPM'de, CJK karakterler 500 CPM'de sayılır
console.log(result.readingTime);
```
