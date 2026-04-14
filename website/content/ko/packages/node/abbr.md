---
title: "abbr()"
description: "약어 정의를 title 속성이 포함된 <abbr> 요소로 변환합니다."
---

```ts
import { abbr } from "@unifast/node";
```

## 시그니처

```ts
function abbr(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// "HTML"의 모든 출현이 <abbr title="Hyper Text Markup Language">로 감싸집니다
```

## 예시

### 기본 약어

`*[TERM]: 정의` 문법으로 약어를 정의합니다. 정의 문단은 출력에서 제거되며, 해당 용어의 모든 출현이 `<abbr>` 요소로 감싸집니다.

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### 여러 개의 약어

여러 약어를 동시에 정의할 수 있습니다. 각 용어는 문서 전체에서 독립적으로 치환됩니다.

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
