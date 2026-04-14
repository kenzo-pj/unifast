---
title: "addClasses()"
description: "CSS 선택자에 일치하는 요소에 CSS 클래스를 추가합니다."
---

```ts
import { addClasses } from "@unifast/node";
```

## 시그니처

```ts
function addClasses(rules: Record<string, string>): UnifastPlugin
```

## 매개변수

### rules

`Record<string, string>` 형태이며, 키는 CSS 선택자이고 값은 일치하는 요소에 추가할 공백으로 구분된 클래스 이름입니다. 클래스는 요소의 기존 클래스와 병합됩니다.

### 지원하는 선택자

다음을 포함해 CSS Selectors Level 4의 상당 부분을 지원합니다.

- **태그 선택자**: `h1`, `p`, `table`
- **클래스 선택자**: `.info`, `.alert.warning`
- **ID 선택자**: `#main`
- **전체 선택자**: `*`
- **속성 선택자**: `[data-type]`, `[href^="https"]`, `[href$=".pdf"]`, `[href*="example"]`, `[class~="bar"]`, `[lang|="en"]`
- **결합자**: 자손(` `), 자식(`>`), 인접 형제(`+`), 일반 형제(`~`)
- **의사 클래스**: `:first-child`, `:last-child`, `:nth-child()`, `:not()`, `:empty`
- **콤마로 구분된 선택자**: `h1, h2, h3`
- **복합 선택자**: `div.alert#main[data-type="warning"]`

## 사용법

```ts
import { compile, addClasses } from "@unifast/node";

const md = `
# Hello World

Some paragraph text.

| Name | Value |
|------|-------|
| A    | 1     |
`;

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-3xl font-bold",
      p: "leading-relaxed",
      table: "border-collapse w-full",
    }),
  ],
});

// <h1 class="text-3xl font-bold">Hello World</h1>
// <p class="leading-relaxed">Some paragraph text.</p>
// <table class="border-collapse w-full">...</table>
```

## 예시

### 콤마로 구분된 선택자

동일한 클래스를 여러 요소 타입에 적용합니다.

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "h1, h2, h3": "font-bold tracking-tight",
    }),
  ],
});
```

### 복잡한 선택자

결합자와 의사 클래스를 사용해 정교하게 대상을 지정합니다.

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "pre > code": "block overflow-x-auto",
      "ul > li:first-child": "mt-0",
      "ul > li:last-child": "mb-0",
      "a[href^=\"https\"]": "external-link",
      "div:not(.alert)": "default-container",
    }),
  ],
});
```

### Tailwind CSS 유틸리티 클래스

`addClasses`로 Markdown에서 생성된 HTML에 Tailwind 유틸리티를 적용하는 것은 흔한 패턴입니다.

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      h1: "text-4xl font-extrabold text-gray-900 mb-8",
      h2: "text-2xl font-bold text-gray-800 mt-12 mb-4",
      h3: "text-xl font-semibold text-gray-700 mt-8 mb-3",
      p: "text-base leading-7 text-gray-600 mb-4",
      a: "text-blue-600 underline hover:text-blue-800",
      blockquote: "border-l-4 border-gray-300 pl-4 italic text-gray-500",
      table: "min-w-full divide-y divide-gray-200",
      "thead th": "px-4 py-2 text-left text-sm font-semibold text-gray-900",
      "tbody td": "px-4 py-2 text-sm text-gray-700",
      "tbody tr:nth-child(2n)": "bg-gray-50",
      img: "rounded-lg shadow-md",
      pre: "rounded-lg overflow-hidden",
      "pre > code": "block p-4 text-sm",
    }),
  ],
});
```

### 속성 선택자

요소의 속성을 기준으로 대상을 지정합니다.

```ts
import { compile, addClasses } from "@unifast/node";

const result = compile(md, {
  plugins: [
    addClasses({
      "[href$=\".pdf\"]": "pdf-link",
      "[href^=\"https\"]": "external",
      "[data-type]": "has-type",
    }),
  ],
});
```
