---
title: "extractLang()"
description: "從 code 元素的 className 中擷取程式語言識別字。"
---

```ts
import { extractLang } from "@unifast/core";
```

## 函式簽名

```ts
function extractLang(code: HastElement): string | null
```

## 參數

### code

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 節點型別識別字 |
| `tagName` | `string` | — | HTML 標籤名稱（通常為 `"code"`） |
| `properties` | `Record<string, unknown>` | — | 元素屬性，其中包含 `className` |
| `children` | `HastNode[]` | — | 元素的子節點 |

## 回傳值

`string | null` — 從第一個 `language-*` class 中擷取出的語言識別字；若找不到語言 class 則為 `null`。

## 用法

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

## 範例

### 從 code 元素擷取語言

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

### 沒有語言 class 時

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

### 當 className 並非陣列時

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

### 搭配 findCodeChild 使用

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
