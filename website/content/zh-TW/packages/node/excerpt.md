---
title: "excerpt()"
description: "從文件內容中擷取摘要。"
---

```ts
import { excerpt } from "@unifast/node";
```

## 函式簽名

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## 參數

### options?

摘要擷取的設定

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | 用來區隔摘要與其餘內容的註解標記 |
| `fallbackParagraphs` | `number` | `1` | 找不到分隔標記時，要做為摘要的開頭段落數 |
| `fallbackCharacters` | `number` | `undefined` | 備援摘要的最大字元長度（會在詞界處截斷） |

同時設定 `fallbackParagraphs` 與 `fallbackCharacters` 時，`fallbackParagraphs` 優先生效。

## 回傳值

此外掛會在 compile 結果中加入 `excerpt` 屬性：

| 屬性 | 型別 | 說明 |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | 從文件中擷取出的純文字摘要 |

## 用法

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

## 範例

### 搭配分隔標記

在 Markdown 中放置 `<!-- more -->` 註解，即可明確標示摘要的結束位置：

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

### 備援為前幾段

當找不到分隔標記時，外掛會退回擷取前 N 個段落：

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

### 備援為字元上限

將摘要截斷至指定的最大字元長度，並在詞界處斷開：

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
