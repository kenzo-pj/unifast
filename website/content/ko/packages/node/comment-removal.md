---
title: "commentRemoval()"
description: "출력에서 HTML 주석을 제거합니다."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## 시그니처

```ts
function commentRemoval(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// HTML 주석이 출력에서 제거됩니다
```

## 예시

### 기본 주석 제거

모든 HTML 주석 노드(`<!-- ... -->`)가 출력 트리에서 제거됩니다. blockquote 같은 블록 요소 안에 중첩된 주석도 함께 제거됩니다.

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### 주석이 아닌 HTML은 유지

주석 노드만 제거되며, 그 밖의 인라인 HTML은 건드리지 않습니다.

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
