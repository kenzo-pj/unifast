---
title: "imgLazyLoading()"
description: 'Thêm thuộc tính loading="lazy" cho hình ảnh để load chậm.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Chữ ký

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Tham số

### options?

Cấu hình cho hành vi lazy loading

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Số hình ảnh cần bỏ qua (ví dụ: bỏ qua hero image) |

Plugin sẽ thêm cả hai thuộc tính `loading="lazy"` và `decoding="async"` vào các phần tử `<img>` khớp, bao gồm cả hình ảnh nằm bên trong các phần tử khác.

## Cách dùng

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

## Ví dụ

### Bỏ qua hình ảnh đầu tiên (mẫu hero image)

Hình ảnh đầu tiên trên một trang thường là hero hoặc banner và cần được load ngay lập tức. Sử dụng `skipFirst` để loại nó khỏi danh sách lazy loading:

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

### Bỏ qua nhiều hình ảnh phía trên fold

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
