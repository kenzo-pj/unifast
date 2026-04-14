---
title: "customHeadingId()"
description: "`{#custom-id}` 構文を使って、見出しにカスタム ID を設定します。"
---

```ts
import { customHeadingId } from "@unifast/node";
```

## シグネチャ

```ts
function customHeadingId(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## 使用例

### カスタム ID

見出しの末尾に `{#custom-id}` 構文を書くと、任意の `id` 属性を設定できます。ブレースで囲まれた部分はレンダリングされたテキストから除去されます。

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### カスタム ID のない見出し

`{#...}` 構文のない見出しは変更されません。見出しテキストから生成されたデフォルトのスラグが使われます。

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### クラスや任意の属性

ブレース構文は `.class` や `key=value` 記法にも対応しています。

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
