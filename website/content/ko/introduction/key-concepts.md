---
title: "핵심 개념"
description: "unifast의 다단계 컴파일 파이프라인, 빌트인 패스, 그리고 MdAst와 HAst 변환의 동작 방식을 이해합니다."
---

unifast는 Markdown을 다단계 파이프라인으로 컴파일합니다. 이 단계들을 이해하면 컴파일러를 설정하고 알맞은 플러그인을 선택하는 데 도움이 됩니다.

### 컴파일 파이프라인

```
입력 텍스트
  → Parse (Markdown 또는 MDX)
  → IR0: MdAst (Markdown 구조)
  → Normalize + 빌트인 MdAst 패스
  → IR1: HAst (HTML 구조)로 로우어링
  → HAst 패스 (sanitize, highlight 등)
  → Emit (HTML 문자열)
```

각 단계는 중간 표현(IR)을 통해 문서를 변환합니다.

### 중간 표현

| IR | 이름 | 용도 |
|----|------|---------|
| **IR0** | MdAst | Markdown 구조 - 제목, 문단, 목록, 코드 블록 |
| **IR1** | HAst | HTML 구조 - 요소, 속성, 텍스트 노드 |
| **IR2** | JsAst | MDX 전용 - ESM import와 JSX 표현식 |

파서는 **MdAst**를 만들고, 이를 다시 HTML 출력을 위한 **HAst**로 로우어링합니다. MDX 입력의 경우 JavaScript 출력을 위한 **JsAst** 노드가 추가로 생성됩니다.

### 패스

패스는 특정 단계에서 AST에 적용되는 변환입니다. unifast는 자주 쓰이는 작업을 위한 빌트인 패스를 제공합니다.

**MdAst 패스** (HTML로 로우어링하기 전에 실행):

- **Normalize** - 이후 패스를 위해 일관된 구조로 정리
- **Slug** - 제목 텍스트로부터 제목 ID 생성
- **TOC** - 제목으로부터 목차 추출
- **Definition Resolution** - 링크/이미지 참조 정의 해석

**HAst 패스** (HTML로 로우어링한 후에 실행):

- **Sanitize** - 허용되지 않은 HTML 요소와 속성 제거
- **Highlight** - 코드 블록에 구문 강조 적용
- **Cleanup** - 불필요한 노드와 공백 제거

패스는 단계별로 정렬되어 실행되므로, 실행 순서를 직접 신경 쓸 필요가 없습니다.

### 플러그인

플러그인은 빌트인 패스를 설정하는 TypeScript 패키지입니다. 컴파일 중에 임의의 JavaScript 코드를 실행하지 않고, 대신 Rust 코어가 문서를 처리하는 방식을 제어할 옵션을 지정합니다.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

각 플러그인은 설정 객체를 반환하며, 이는 Rust 코어가 실행되기 전에 컴파일 옵션에 병합됩니다. 이렇게 해서 핫 패스(hot path)를 전적으로 네이티브 코드에 남겨 둡니다.

### 출력 형식

컴파일러는 `outputKind` 옵션을 통해 다양한 출력 형식을 지원합니다.

| 형식 | 설명 |
|--------|-------------|
| `"html"` | HTML 문자열 (기본값) |
| `"hast"` | HAst JSON - 커스텀 렌더링을 위한 HTML AST |
| `"mdast"` | MdAst JSON - 분석용 Markdown AST |
| `"mdx-js"` | JavaScript 모듈 문자열 (MDX 전용) |

### 진단(Diagnostics)

컴파일러는 결과 객체의 `diagnostics` 배열을 통해 문제를 보고합니다. 각 진단 정보에는 심각도, 메시지, 그리고 정확한 오류 위치를 나타내는 선택적 소스 스팬이 포함됩니다.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
