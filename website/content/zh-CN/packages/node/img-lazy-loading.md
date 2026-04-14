---
title: "imgLazyLoading()"
description: '为图片添加 loading="lazy" 属性以实现延迟加载。'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## 签名

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## 参数

### options?

懒加载行为的配置

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | 跳过前若干张图片（例如跳过首屏主图） |

该插件会为匹配到的 `<img>` 元素同时添加 `loading="lazy"` 和 `decoding="async"` 属性，包括嵌套在其他元素内部的图片。

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

## 示例

### 跳过首张图片（首屏主图模式）

页面上的第一张图片通常是首屏主图或横幅，应当立即加载。可以使用 `skipFirst` 将它排除在懒加载之外：

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

### 跳过多张首屏可见图片

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
