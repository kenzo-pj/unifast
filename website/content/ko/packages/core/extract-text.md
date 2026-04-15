---
title: "extractText()"
description: "HAST 노드에서 모든 텍스트 콘텐츠를 재귀적으로 추출합니다."
---

```ts
import { extractText } from "@unifast/core";
```

## 시그니처

```ts
function extractText(node: HastNode): string
```

## 매개변수

### node

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `type` | `string` | — | 노드 타입 (`"root"`, `"element"`, `"text"` 등) |
| `children` | `HastNode[]` | — | 자식 노드 (`"root"` 및 `"element"` 타입용) |
| `value` | `string` | — | 텍스트 내용 (`"text"` 타입용) |

## 반환값

`string` — 노드와 그 자손에서 추출한 모든 텍스트가 이어 붙은 문자열.

## 사용법

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const element: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "Hello " },
    {
      type: "element",
      tagName: "strong",
      properties: {},
      children: [{ type: "text", value: "world" }],
    },
  ],
};

const text = extractText(element);

console.log(text);
// Hello world
```

## 예시

### 단순한 요소에서 추출

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h1",
  properties: { id: "title" },
  children: [{ type: "text", value: "Getting Started" }],
};

console.log(extractText(heading));
// Getting Started
```

### 중첩된 요소에서 추출

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const paragraph: HastElement = {
  type: "element",
  tagName: "p",
  properties: {},
  children: [
    { type: "text", value: "This is " },
    {
      type: "element",
      tagName: "em",
      properties: {},
      children: [
        { type: "text", value: "deeply " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "nested" }],
        },
      ],
    },
    { type: "text", value: " content." },
  ],
};

console.log(extractText(paragraph));
// This is deeply nested content.
```

### 빈 요소

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const emptyDiv: HastElement = {
  type: "element",
  tagName: "div",
  properties: {},
  children: [],
};

console.log(extractText(emptyDiv));
// (빈 문자열)
```

### 제목 슬러그 생성

```ts
import { extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const heading: HastElement = {
  type: "element",
  tagName: "h2",
  properties: {},
  children: [
    { type: "text", value: "API " },
    {
      type: "element",
      tagName: "code",
      properties: {},
      children: [{ type: "text", value: "Reference" }],
    },
  ],
};

const slug = extractText(heading).toLowerCase().replace(/\s+/g, "-");

console.log(slug);
// api-reference
```
