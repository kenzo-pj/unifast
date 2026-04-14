---
title: "excerpt()"
description: "문서 내용에서 요약을 추출합니다."
---

```ts
import { excerpt } from "@unifast/node";
```

## 시그니처

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## 매개변수

### options?

요약 추출 설정

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | 요약과 나머지를 구분하는 주석 마커 |
| `fallbackParagraphs` | `number` | `1` | 구분자가 없을 때 요약으로 사용할 선두 문단 수 |
| `fallbackCharacters` | `number` | `undefined` | 폴백 요약의 최대 문자 수 (단어 경계에서 잘림) |

`fallbackParagraphs`와 `fallbackCharacters`가 모두 지정되면 `fallbackParagraphs`가 우선 적용됩니다.

## 반환값

플러그인은 컴파일 결과에 `excerpt` 속성을 추가합니다.

| 속성 | 타입 | 설명 |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | 문서에서 추출한 평문 요약 |

## 사용법

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

## 예시

### 구분자 마커 사용

Markdown 안에 `<!-- more -->` 주석을 배치하여 요약이 끝나는 지점을 명시적으로 표시할 수 있습니다.

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

### 첫 문단으로 폴백

구분자 마커가 없으면 플러그인은 앞에서부터 N개의 문단을 추출하는 방식으로 폴백합니다.

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

### 문자 수 제한으로 폴백

요약을 최대 문자 수로 자르되, 단어 경계에서 끊어 줍니다.

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
