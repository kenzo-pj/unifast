---
title: "escapeHtml()"
description: "문자열의 HTML 특수 문자를 이스케이프합니다."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## 시그니처

```ts
function escapeHtml(str: string): string
```

## 매개변수

### str

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `str` | `string` | — | 이스케이프할 문자를 포함한 문자열 |

## 반환값

`string` — `&`, `<`, `>`, `"`가 각각의 HTML 엔티티로 치환된 문자열.

## 사용법

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## 예시

### 기본 이스케이프

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### 사용자 생성 콘텐츠 이스케이프

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### 안전한 HTML 속성 만들기

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## 동작

- **`&`**는 `&amp;`로 치환됩니다.
- **`<`**는 `&lt;`로 치환됩니다.
- **`>`**는 `&gt;`로 치환됩니다.
- **`"`**는 `&quot;`로 치환됩니다.
- 그 외의 문자는 그대로 유지됩니다.
