---
title: "imgLazyLoading()"
description: '画像に loading="lazy" 属性を追加して読み込みを遅延させます。'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## シグネチャ

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## パラメータ

### options?

遅延読み込みの動作設定。

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | スキップする画像の数 (例: ヒーロー画像をスキップ) |

プラグインは、他の要素内にネストされた画像も含め、マッチした `<img>` 要素に `loading="lazy"` と `decoding="async"` の両方の属性を追加します。

## 使い方

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Photo 1](photo1.jpg)

![Photo 2](photo2.jpg)

![Photo 3](photo3.jpg)
`;

const result = compile(md, {
  plugins: [imgLazyLoading()],
});

// All images get loading="lazy" and decoding="async":
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## 使用例

### 最初の画像をスキップする (ヒーロー画像のパターン)

ページ内の最初の画像は、即座に読み込みたいヒーロー画像やバナー画像であることが多いです。`skipFirst` を使って遅延読み込みの対象から除外できます。

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Hero banner](hero.jpg)

Some introductory content...

![Diagram](diagram.jpg)

More content...

![Screenshot](screenshot.jpg)
`;

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 1,
    }),
  ],
});

// First image loads eagerly (no loading attribute):
// <img src="hero.jpg" alt="Hero banner">
//
// Remaining images are lazy loaded:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### ファーストビュー内の複数画像をスキップする

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // skip the first 3 images
    }),
  ],
});
```
