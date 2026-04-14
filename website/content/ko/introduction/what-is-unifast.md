---
title: "unifast란?"
description: "unifast는 Rust 코어 기반의 고성능 Markdown/MDX 컴파일러입니다. GFM, 새니타이즈, 구문 강조, TOC를 빌트인 패스로 제공합니다."
---

unifast는 Rust 코어로 동작하는 고성능 Markdown/MDX 컴파일러입니다. JS 플러그인 호환성을 추구하는 대신, remark/rehype가 다루는 주요 사용 사례를 빌트인 패스로 직접 구현하여 제공합니다.

### unifast가 필요한 이유

unified/remark/rehype 같은 전통적인 Markdown 툴체인은 강력하지만 몇 가지 타협이 따릅니다.

- **성능 오버헤드** - 여러 단계의 JS AST 변환이 누적되며, 대규모 처리에서 그 비용이 특히 커집니다.
- **플러그인 조율** - 수십 개의 플러그인 간 순서, 호환성, 중복을 관리해야 합니다.
- **기본 기능 부재** - GFM이나 새니타이즈 같은 기본적인 작업조차 별도 패키지가 필요합니다.

unifast는 다른 방식을 택합니다.

- **Rust 코어** - 파싱, 변환, 출력이 모두 네이티브 코드에서 이루어집니다.
- **빌트인 패스** - GFM, 새니타이즈, 구문 강조, TOC 등 자주 쓰이는 기능을 외부에 덧붙이지 않고 기본으로 내장합니다.
- **단일 컴파일** - 한 번의 호출로 Markdown을 HTML로 컴파일하면서 모든 기능이 함께 적용됩니다.

### 주요 기능

| 기능 | 설명 |
|---------|-------------|
| **CommonMark + GFM** | 표, 작업 목록, 취소선, 자동 링크, 각주 |
| **Frontmatter** | YAML, TOML, JSON 메타데이터 추출 |
| **MDX** | Markdown 안에서 JSX 표현식과 import 사용 |
| **Diagnostics** | 라인/컬럼 정보를 포함한 정밀한 오류 위치 |

### 빌트인 패스

주요 remark/rehype 플러그인이 네이티브 Rust 패스로 재구현되어 있습니다. npm 설치도, 순서 고민도 필요 없습니다.

| 패스 | 설명 |
|------|-------------|
| **Sanitization** | 스키마 기반 HTML 허용 목록과 안전한 기본값 |
| **Syntax Highlighting** | 교체 가능한 엔진(syntect, Shiki) |
| **Table of Contents** | 제목 트리를 자동으로 추출 |

### 플랫폼 지원

unifast는 하나의 Rust 코어에서 여러 플랫폼으로 확장됩니다.

- **`@unifast/node`** - N-API(napi-rs)를 통한 Node.js 바인딩. 주요 타깃입니다.
- **`@unifast/core`** - 모든 패키지가 공유하는 TypeScript 타입 정의.
- **`unifast` (CLI)** - 스크립트와 CI를 위한 커맨드라인 인터페이스.
- **WASM** - 브라우저 및 엣지 런타임 지원(보조 타깃).

### 지향하지 않는 것

unifast는 unified의 **드롭인 대체재가 아닙니다**. 다음 기능은 제공하지 않습니다.

- 기존 remark/rehype JS 플러그인을 코어 내부에서 실행하는 것.
- unified 생태계와의 API 호환성.
- 코어 컴파일 경로에서 Node의 모듈 해석에 의존하는 것.

대신 unifast는 **사용 사례의 완성도**를 목표로 합니다. 플러그인 파이프라인을 직접 조립하는 복잡함 없이, 대부분의 프로젝트가 필요로 하는 기능을 기본으로 제공합니다.
