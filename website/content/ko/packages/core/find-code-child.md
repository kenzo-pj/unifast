---
title: "findCodeChild()"
description: "부모 요소 안에서 자식 <code> 요소를 찾습니다."
---

```ts
import { findCodeChild } from "@unifast/core";
```

## 시그니처

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## 매개변수

### element

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 노드 타입 식별자 |
| `tagName` | `string` | — | HTML 태그 이름 (일반적으로 `"pre"`) |
| `properties` | `Record<string, unknown>` | — | 요소 속성 |
| `children` | `HastNode[]` | — | 탐색할 자식 노드 |

## 반환값

`HastElement | undefined` — `tagName`이 `"code"`인 첫 번째 자식 요소. 그러한 자식이 없으면 `undefined`.

## 사용법

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-js"] },
      children: [{ type: "text", value: "const x = 1;" }],
    },
  ],
};

const code = findCodeChild(pre);

console.log(code?.tagName);
// code
```

## 예시

### pre 요소 안에서 code 찾기

```ts
import { findCodeChild, extractLang, extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-rust"] },
      children: [{ type: "text", value: 'fn main() { println!("hello"); }' }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // rust
  console.log(extractText(code));
  // fn main() { println!("hello"); }
}
```

### code 자식이 없는 경우

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    { type: "text", value: "plain preformatted text" },
  ],
};

const code = findCodeChild(pre);

console.log(code);
// undefined
```

### visitHast와 함께 구문 강조에 사용

```ts
import { visitHast, findCodeChild, extractLang } from "@unifast/core";
import type { HastNode, HastElement } from "@unifast/core";

const tree: HastNode = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    const code = findCodeChild(node);
    if (code) {
      const lang = extractLang(code);
      console.log(`Found code block with language: ${lang}`);
      // Found code block with language: js
    }
  }
});
```
