---
title: "imgLazyLoading()"
description: '지연 로딩을 위해 이미지에 loading="lazy" 속성을 추가합니다.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## 시그니처

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## 매개변수

### options?

지연 로딩 동작 설정

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | 건너뛸 이미지 수 (예: 히어로 이미지 건너뛰기) |

플러그인은 다른 요소 안에 중첩된 이미지를 포함해, 매칭되는 `<img>` 요소에 `loading="lazy"`와 `decoding="async"` 속성을 함께 추가합니다.

## 사용법

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

// 모든 이미지가 loading="lazy"와 decoding="async"를 가집니다:
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## 예시

### 첫 이미지 건너뛰기 (히어로 이미지 패턴)

페이지의 첫 이미지는 보통 즉시 로드되어야 하는 히어로 또는 배너 이미지입니다. `skipFirst`를 사용해 지연 로딩에서 제외할 수 있습니다.

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

// 첫 이미지는 즉시 로드됩니다(loading 속성 없음):
// <img src="hero.jpg" alt="Hero banner">
//
// 나머지 이미지는 지연 로딩됩니다:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### 첫 화면의 여러 이미지 건너뛰기

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // 처음 세 개의 이미지를 건너뜀
    }),
  ],
});
```
