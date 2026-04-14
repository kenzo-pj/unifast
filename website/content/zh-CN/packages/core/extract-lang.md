---
title: "extractLang()"
description: "从 code 元素的 className 中提取编程语言。"
---

```ts
import { extractLang } from "@unifast/core";
```

## 签名

```ts
function extractLang(code: HastElement): string | null
```

## 参数

### code

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | 节点类型标识 |
| `tagName` | `string` | — | HTML 标签名（通常是 `"code"`） |
| `properties` | `Record<string, unknown>` | — | 元素属性，包括 `className` |
| `children` | `HastNode[]` | — | 元素的子节点 |

## 返回值

`string | null` —— 从首个 `language-*` 类名中提取出的语言标识；若未找到语言类名，则返回 `null`。

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

## 示例

### 从 code 元素中提取语言

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

### 没有语言类名的情况

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

### className 不是数组的情况

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

### 与 findCodeChild 配合使用

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
