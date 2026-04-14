---
title: "readingTime()"
description: "ドキュメントの読了時間を推定して compile の結果に含めます。"
---

```ts
import { readingTime } from "@unifast/node";
```

## シグネチャ

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## パラメータ

### options?

読了時間の推定設定。

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | ラテン文字テキストの 1 分あたりの単語数 |
| `cjkCharsPerMinute` | `number` | `500` | CJK テキストの 1 分あたりの文字数 |

## 戻り値

プラグインは compile の結果に `readingTime` プロパティを追加します。

| プロパティ | 型 | 説明 |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | 分単位の推定読了時間 (最小値 1、0.5 刻みで切り上げ) |
| `result.readingTime.words` | `number` | 単語数の合計 (ラテン単語 + CJK 文字数) |

## 使い方

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

## 使用例

### カスタムの 1 分あたりの単語数

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

### CJK コンテンツ

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

### ラテン文字と CJK の混在テキスト

読了時間はラテン単語と CJK 文字について別々に計算され、その後に合算されます。コードブロックは単語数のカウントから除外されます。

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
