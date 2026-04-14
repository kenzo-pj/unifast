---
title: "imgLazyLoading()"
description: 'Добавляет атрибут loading="lazy" к изображениям для отложенной загрузки.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Сигнатура

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Параметры

### options?

Конфигурация поведения отложенной загрузки

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Количество пропускаемых изображений (например, пропустить hero-изображение) |

Плагин добавляет атрибуты `loading="lazy"` и `decoding="async"` к соответствующим элементам `<img>`, включая изображения, вложенные в другие элементы.

## Использование

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

// Все изображения получают loading="lazy" и decoding="async":
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## Примеры

### Пропуск первого изображения (паттерн hero-изображения)

Первое изображение на странице часто является hero или баннером, которое должно загружаться сразу. Используйте `skipFirst`, чтобы исключить его из отложенной загрузки:

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

// Первое изображение загружается сразу (без атрибута loading):
// <img src="hero.jpg" alt="Hero banner">
//
// Остальные изображения загружаются отложенно:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### Пропуск нескольких изображений над сгибом

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // пропустить первые 3 изображения
    }),
  ],
});
```
