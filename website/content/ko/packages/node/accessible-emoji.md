---
title: "accessibleEmoji()"
description: '접근성을 위해 이모지 문자를 aria-label이 포함된 <span role="img"> 요소로 감쌉니다.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## 시그니처

```ts
function accessibleEmoji(): UnifastPlugin
```

## 매개변수

없음.

## 사용법

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// 이모지가 <span role="img" aria-label="rocket">으로 감싸집니다
```

## 예시

### aria 레이블이 포함된 이모지 감싸기

각 이모지 문자는 `role="img"`과 스크린 리더를 위한 `aria-label`이 지정된 `<span>`으로 감싸집니다.

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### 이모지 없는 텍스트

이모지가 없는 순수 텍스트는 그대로 통과합니다.

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
