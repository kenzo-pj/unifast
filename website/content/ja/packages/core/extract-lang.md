---
title: "extractLang()"
description: "code 要素の className からプログラミング言語を抽出します。"
---

```ts
import { extractLang } from "@unifast/core";
```

## シグネチャ

```ts
function extractLang(code: HastElement): string | null
```

## パラメータ

### code

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | ノード種別の識別子 |
| `tagName` | `string` | — | HTML のタグ名 (通常は `"code"`) |
| `properties` | `Record<string, unknown>` | — | `className` を含む要素のプロパティ |
| `children` | `HastNode[]` | — | 要素の子ノード |

## 戻り値

`string | null` — 最初の `language-*` クラスから抽出された言語識別子です。言語クラスが見つからない場合は `null` を返します。

## 使い方

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const codeElement: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-typescript"] },
  children: [{ type: "text", value: "const x = 1;" }],
};

const lang = extractLang(codeElement);

console.log(lang);
// typescript
```

## 使用例

### code 要素から言語を抽出する

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["language-js", "highlight"] },
  children: [{ type: "text", value: "console.log('hello');" }],
};

console.log(extractLang(code));
// js
```

### 言語クラスがない場合

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: { className: ["highlight"] },
  children: [{ type: "text", value: "plain text" }],
};

console.log(extractLang(code));
// null
```

### className が配列でない場合

```ts
import { extractLang } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const code: HastElement = {
  type: "element",
  tagName: "code",
  properties: {},
  children: [{ type: "text", value: "no classes" }],
};

console.log(extractLang(code));
// null
```

### findCodeChild と組み合わせて使う

```ts
import { extractLang, findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-python"] },
      children: [{ type: "text", value: "print('hello')" }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // python
}
```
