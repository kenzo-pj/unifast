---
title: "visitHast()"
description: "비지터 함수를 이용해 HAST 트리를 순회하고 변환합니다."
---

```ts
import { visitHast } from "@unifast/core";
```

## 시그니처

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## 매개변수

### node

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `type` | `string` | — | 노드 타입 (`"root"`, `"element"`, `"text"` 등) |
| `children` | `HastNode[]` | — | 자식 노드 (`"root"` 및 `"element"` 타입용) |

### visitor

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | 각 노드에 대해 호출되는 함수. 노드를 반환하면 기존 노드를 대체하며, `void`를 반환하면 그대로 유지합니다. |

## 반환값

`HastNode` — 비지터 함수가 적용된 새 트리.

## 사용법

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// 텍스트가 "HELLO WORLD"로 바뀐 트리
```

## 예시

### 모든 문단에 클래스 추가

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// 두 <p> 요소 모두 className: ["prose"]를 갖게 됩니다
```

### 모든 이미지 제거

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// 트리에서 <img> 요소가 제거됩니다
```

### 모든 링크 수집

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### 코드 블록을 컨테이너로 감싸기

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre>가 <div class="code-block"> 안에 감싸집니다
```

## 동작

- **불변성:** 원본 트리를 수정하지 않고 새 트리를 반환합니다.
- **하향식 순회:** 비지터는 자식보다 부모에서 먼저 호출됩니다.
- **치환:** 비지터가 노드를 반환하면, 자식을 순회하기 전에 현재 노드를 대체합니다.
- **변경 없음:** 비지터가 `void`(또는 `undefined`)를 반환하면 원래 노드를 유지합니다.
- **재귀적:** `"root"`와 `"element"` 노드의 자식은 재귀적으로 순회됩니다.
