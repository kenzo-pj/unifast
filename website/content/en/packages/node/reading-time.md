---
title: "readingTime()"
description: "Estimate the reading time of the document and include it in the compile result."
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

Configuration for reading time estimation

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Words per minute for Latin text |
| `cjkCharsPerMinute` | `number` | `500` | Characters per minute for CJK text |

## Returns

The plugin adds a `readingTime` property to the compile result:

| Property | Type | Description |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Estimated reading time in minutes (minimum 1, rounded up to nearest 0.5) |
| `result.readingTime.words` | `number` | Total word count (Latin words + CJK characters) |

## Usage

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

## Examples

### Custom words per minute

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
      cjkCharsPerMinute: 400, // adjust for CJK reading speed
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Mixed Latin and CJK text

Reading time is calculated separately for Latin words and CJK characters, then combined. Code blocks are excluded from the word count.

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
