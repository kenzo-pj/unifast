---
title: "hastToHtml()"
description: "HAST 루트 노드를 HTML 문자열로 직렬화합니다."
---

```ts
import { hastToHtml } from "@unifast/core";
```

## 시그니처

```ts
function hastToHtml(hast: HastRoot): string
```

## 매개변수

### hast

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `type` | `"root"` | — | 노드 타입 식별자 |
| `children` | `HastNode[]` | — | 트리의 자식 노드 |

## 반환값

`string` — 직렬화된 HTML 문자열.

## 사용법

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## 예시

### 기본 직렬화

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### 빈(void) 요소

`<br>`, `<img>`, `<hr>` 등의 빈 요소는 자동으로 자체 닫힘 형태로 출력됩니다.

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### compile() 출력과 함께 사용

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### 원본 HTML 패스스루

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## 동작

- **HTML 이스케이프:** 텍스트 콘텐츠는 이스케이프됩니다 (`&`, `<`, `>`, `"`).
- **Void 요소:** `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr`은 자체 닫힘 형태로 출력됩니다.
- **속성:** 알파벳 순으로 정렬됩니다. `className` 배열은 공백으로 연결되어 `class`로 렌더링됩니다. boolean `true`는 값 없는 속성으로, `false`/`null`/`undefined`는 생략됩니다.
- **주석:** `<!--value-->` 형태로 렌더링됩니다.
- **Doctype:** `<!DOCTYPE html>`로 렌더링됩니다.
- **Raw 노드:** 이스케이프 없이 그대로 출력됩니다.
