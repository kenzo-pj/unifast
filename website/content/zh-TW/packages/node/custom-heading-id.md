---
title: "customHeadingId()"
description: "以 {#custom-id} 語法為標題設定自訂 ID。"
---

```ts
import { customHeadingId } from "@unifast/node";
```

## 函式簽名

```ts
function customHeadingId(): UnifastPlugin
```

## 參數

無。

## 用法

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## 範例

### 自訂 ID

在標題結尾使用 `{#custom-id}` 語法可指定特定的 `id` 屬性。大括號區塊會從渲染後的文字中移除：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### 未使用自訂 ID 的標題

未使用 `{#...}` 語法的標題不會被修改。它們會使用從標題文字自動產生的預設 slug：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Class 與任意屬性

大括號語法同時支援 `.class` 與 `key=value` 寫法：

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
