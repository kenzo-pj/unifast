---
title: "readingTime()"
description: "문서의 읽기 시간을 추정하여 컴파일 결과에 포함합니다."
---

```ts
import { readingTime } from "@unifast/node";
```

## 시그니처

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## 매개변수

### options?

읽기 시간 추정 설정

| 속성 | 타입 | 기본값 | 설명 |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | 라틴 문자 텍스트의 분당 단어 수 |
| `cjkCharsPerMinute` | `number` | `500` | CJK 텍스트의 분당 문자 수 |

## 반환값

플러그인은 컴파일 결과에 `readingTime` 속성을 추가합니다.

| 속성 | 타입 | 설명 |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | 추정 읽기 시간(분). 최소 1분이며 0.5 단위로 올림 |
| `result.readingTime.words` | `number` | 총 단어 수 (라틴 단어 + CJK 문자) |

## 사용법

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# My Article

This is a short article with some content that demonstrates
reading time estimation.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

console.log(result.readingTime);
// { minutes: 1, words: 16 }
```

## 예시

### 커스텀 분당 단어 수

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // 더 느린 읽기 속도
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### CJK 콘텐츠

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // CJK 읽기 속도에 맞게 조정
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### 라틴 문자와 CJK가 섞인 텍스트

라틴 단어와 CJK 문자의 읽기 시간이 각각 계산된 뒤 합산됩니다. 코드 블록은 단어 수 집계에서 제외됩니다.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// 라틴 단어는 200 WPM, CJK 문자는 500 CPM으로 계산됩니다
console.log(result.readingTime);
```
