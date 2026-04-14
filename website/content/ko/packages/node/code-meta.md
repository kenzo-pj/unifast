---
title: "codeMeta()"
description: "코드 펜스의 메타 문자열을 파싱하여 코드 블록의 data 속성으로 변환합니다."
---

```ts
import { codeMeta } from "@unifast/node";
```

## 시그니처

```ts
function codeMeta(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="example.js"
console.log("hello");
\`\`\``;

const result = compile(md, {
  plugins: [codeMeta()],
});
// <pre> 요소가 data-title="example.js"를 갖게 됩니다
```

## 예시

### 기본 메타 파싱

`codeMeta()` 플러그인은 펜스드 코드 블록에서 언어 식별자 다음에 오는 메타 문자열을 파싱하여, 인식되는 키를 `<pre>` 요소의 `data-*` 속성으로 변환합니다.

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`js title="app.ts"
const x = 1;
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
console.log(result.output);
// <pre data-lang="js" data-title="app.ts"><code class="language-js">const x = 1;
// </code></pre>
```

### 여러 메타 속성

`title`, 줄 강조용 `{1,3-5}`, `showLineNumbers`, `diff`, `wordWrap` 등 여러 메타 속성을 조합할 수 있습니다.

```ts
import { compile, codeMeta } from "@unifast/node";

const md = `\`\`\`ts title="server.ts" {1,3} showLineNumbers
import express from "express";
const app = express();
app.listen(3000);
\`\`\``;

const result = compile(md, { plugins: [codeMeta()] });
// <pre> 요소가 다음을 갖게 됩니다:
//   data-title="server.ts"
//   1행과 3행에 data-highlighted 속성이 지정됨
//   showLineNumbers는 불리언 플래그로 인식됨
```
