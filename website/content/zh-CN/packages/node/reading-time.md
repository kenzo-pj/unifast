---
title: "readingTime()"
description: "估算文档的阅读时长，并将其附加到编译结果中。"
---

```ts
import { readingTime } from "@unifast/node";
```

## 签名

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## 参数

### options?

阅读时长估算的配置

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | 拉丁文的每分钟阅读词数 |
| `cjkCharsPerMinute` | `number` | `500` | CJK 文本的每分钟阅读字符数 |

## 返回值

该插件会在编译结果中添加 `readingTime` 属性：

| 属性 | 类型 | 描述 |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | 估算的阅读时长（分钟，最小值为 1，向上取整到最接近的 0.5） |
| `result.readingTime.words` | `number` | 总字数（拉丁词数 + CJK 字符数） |

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

## 示例

### 自定义每分钟词数

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

### CJK 内容

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

### 拉丁文与 CJK 混合文本

阅读时长会分别按照拉丁词数和 CJK 字符数进行计算，然后合并得到最终结果。代码块不会计入字数。

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
