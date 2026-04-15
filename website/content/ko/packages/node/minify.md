---
title: "minify()"
description: "불필요한 공백을 제거하여 HTML 출력을 최소화합니다."
---

```ts
import { minify } from "@unifast/node";
```

## 시그니처

```ts
function minify(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// 공백이 정리되고 불필요한 노드가 제거됩니다
```

## 예시

### 기본 미니파이

`minify()` 플러그인은 연속된 공백 문자를 하나의 공백으로 축소하고, HTML 주석을 제거하며, 블록 요소 사이의 공백만 있는 텍스트 노드를 제거하고, 빈 `class`와 `style` 속성을 삭제합니다.

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### preformatted 콘텐츠는 유지

`<pre>`와 `<code>` 블록 내부의 공백은 그대로 유지되므로, 코드 포맷이 깨지지 않습니다.

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// <pre><code> 블록 내부의 공백은 작성된 그대로 유지됩니다
```
