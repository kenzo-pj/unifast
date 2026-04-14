---
title: "imgLazyLoading()"
description: '為圖片加上 loading="lazy" 屬性以實現延遲載入。'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## 函式簽名

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## 參數

### options?

延遲載入行為的設定

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | 要跳過的圖片數量（例如跳過 hero 圖片） |

此外掛會對符合條件的 `<img>` 元素同時加上 `loading="lazy"` 與 `decoding="async"` 屬性，包含巢狀於其他元素內部的圖片。

## 用法

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

## 範例

### 跳過第一張圖片（hero 圖片模式）

頁面上的第一張圖片通常是 hero 或橫幅圖片，應該立刻載入。使用 `skipFirst` 可將其排除於延遲載入之外：

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

### 跳過多張首屏圖片

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
