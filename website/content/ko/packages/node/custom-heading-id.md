---
title: "customHeadingId()"
description: "{#custom-id} 문법을 사용해 제목에 커스텀 ID를 지정합니다."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## 시그니처

```ts
function customHeadingId(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// 제목이 자동 생성된 슬러그 대신 id="intro"를 갖게 됩니다
```

## 예시

### 커스텀 ID

제목 끝에 `{#custom-id}` 문법을 사용하면 특정 `id` 속성을 지정할 수 있습니다. 중괄호 블록은 렌더링된 텍스트에서 제거됩니다.

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### 커스텀 ID가 없는 제목

`{#...}` 문법이 없는 제목은 그대로 유지됩니다. 제목 텍스트에서 생성된 기본 슬러그를 사용합니다.

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### 클래스와 임의의 속성

중괄호 문법은 `.class`와 `key=value` 표기도 지원합니다.

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
