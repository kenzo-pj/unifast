---
title: "readingTime()"
description: "document के reading time का अनुमान लगाएँ और इसे compile result में शामिल करें।"
---

```ts
import { readingTime } from "@unifast/node";
```

## Signature

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parameters

### options?

Reading time अनुमान के लिए Configuration

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Latin text के लिए Words per minute |
| `cjkCharsPerMinute` | `number` | `500` | CJK text के लिए Characters per minute |

## Returns

Plugin compile result में एक `readingTime` property जोड़ता है:

| Property | Type | विवरण |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | minutes में अनुमानित reading time (न्यूनतम 1, निकटतम 0.5 तक rounded up) |
| `result.readingTime.words` | `number` | कुल word count (Latin words + CJK characters) |

## उपयोग

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

## उदाहरण

### Custom words per minute

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // धीमी reading speed
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### CJK content

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // CJK reading speed के लिए adjust करें
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Mixed Latin और CJK text

Reading time को Latin words और CJK characters के लिए अलग-अलग calculate किया जाता है, फिर combine किया जाता है। Code blocks को word count से बाहर रखा जाता है।

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Latin words 200 WPM पर, CJK characters 500 CPM पर count होते हैं
console.log(result.readingTime);
```
