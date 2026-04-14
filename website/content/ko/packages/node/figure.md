---
title: "figure()"
description: "alt 텍스트가 있는 이미지를 <figure>와 <figcaption> 요소로 감쌉니다."
---

```ts
import { figure } from "@unifast/node";
```

## 시그니처

```ts
function figure(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// 이미지가 <figcaption>과 함께 <figure>로 감싸집니다
```

## 예시

### 기본 figure 감싸기

이미지에 alt 텍스트가 있으면 `figure()`는 이를 `<figure>` 요소로 감싸고, alt 텍스트를 담은 `<figcaption>`을 추가합니다.

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### alt 텍스트가 없는 이미지

alt 텍스트가 없는 이미지는 표시할 의미 있는 캡션이 없으므로 감싸지 않습니다.

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
