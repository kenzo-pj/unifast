---
title: "extractLang()"
description: "코드 요소의 className에서 프로그래밍 언어를 추출합니다."
---

```ts
import { extractLang } from "@unifast/core";
```

## 시그니처

```ts
function extractLang(code: HastElement): string | null
```

## 매개변수

### code

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 노드 타입 식별자 |
| `tagName` | `string` | — | HTML 태그 이름 (일반적으로 `"code"`) |
| `properties` | `Record<string, unknown>` | — | `className`을 포함한 요소 속성 |
| `children` | `HastNode[]` | — | 요소의 자식 노드 |

## 반환값

`string | null` — 첫 번째 `language-*` 클래스에서 추출한 언어 식별자. 언어 클래스가 없으면 `null`.

## 사용법

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const codeElement: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-typescript"] },
  children: [{ type: "text", value: "const x = 1;" }],
};

const lang = extractLang(codeElement);

console.log(lang);
// typescript
```

## 예시

### 코드 요소에서 언어 추출

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-js", "highlight"] },
  children: [{ type: "text", value: "console.log('hello');" }],
};

console.log(extractLang(code));
// js
```

### 언어 클래스가 없는 경우

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["highlight"] },
  children: [{ type: "text", value: "plain text" }],
};

console.log(extractLang(code));
// null
```

### className이 배열이 아닌 경우

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: {},
  children: [{ type: "text", value: "no classes" }],
};

console.log(extractLang(code));
// null
```

### findCodeChild와 함께 사용

```ts
import { extractLang, findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python"] },
      children: [{ type: "text", value: "print('hello')" }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // python
}
```
