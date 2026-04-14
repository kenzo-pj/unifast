---
title: "excerpt()"
description: "ドキュメントの内容から抜粋を抽出します。"
---

```ts
import { excerpt } from "@unifast/node";
```

## シグネチャ

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## パラメータ

### options?

抜粋抽出の設定。

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | 抜粋とそれ以降を区切るコメントマーカー |
| `fallbackParagraphs` | `number` | `1` | 区切りマーカーが見つからない場合に抜粋として使う先頭の段落数 |
| `fallbackCharacters` | `number` | `undefined` | フォールバック抜粋の最大文字数 (単語境界で切り詰めます) |

`fallbackParagraphs` と `fallbackCharacters` の両方が設定されている場合、`fallbackParagraphs` が優先されます。

## 戻り値

プラグインは compile の結果に `excerpt` プロパティを追加します。

| プロパティ | 型 | 説明 |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | ドキュメントから抽出されたプレーンテキストの抜粋 |

## 使い方

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

## 使用例

### 区切りマーカーを使う

Markdown に `<!-- more -->` コメントを配置することで、抜粋の終わりを明示的にマークできます。

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

### 先頭の段落へのフォールバック

区切りマーカーが見つからない場合、プラグインは先頭の N 段落を抽出するフォールバックに切り替わります。

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

### 文字数制限へのフォールバック

単語境界で区切りながら、抜粋を最大文字数まで切り詰めます。

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
