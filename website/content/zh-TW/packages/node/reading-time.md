---
title: "readingTime()"
description: "估算文件的閱讀時間，並將結果納入 compile 的回傳值。"
---

```ts
import { readingTime } from "@unifast/node";
```

## 函式簽名

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## 參數

### options?

閱讀時間估算的設定

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | 拉丁文字每分鐘可讀的字數 |
| `cjkCharsPerMinute` | `number` | `500` | CJK 文字每分鐘可讀的字元數 |

## 回傳值

此外掛會在 compile 結果中加入 `readingTime` 屬性：

| 屬性 | 型別 | 說明 |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | 估算的閱讀時間（分鐘，最少為 1，並會向上進位至最接近的 0.5） |
| `result.readingTime.words` | `number` | 總字數（拉丁字數加上 CJK 字元數） |

## 用法

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

## 範例

### 自訂每分鐘字數

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

### CJK 內容

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

### 拉丁與 CJK 混合文字

閱讀時間會分別針對拉丁字數與 CJK 字元計算後再相加。程式碼區塊則不會納入字數統計。

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
