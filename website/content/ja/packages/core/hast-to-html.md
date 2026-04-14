---
title: "hastToHtml()"
description: "HAST のルートノードを HTML 文字列にシリアライズします。"
---

```ts
import { hastToHtml } from "@unifast/core";
```

## シグネチャ

```ts
function hastToHtml(hast: HastRoot): string
```

## パラメータ

### hast

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `type` | `"root"` | — | ノード種別の識別子 |
| `children` | `HastNode[]` | — | ツリーの子ノード |

## 戻り値

`string` — シリアライズされた HTML 文字列です。

## 使い方

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "h1",
      properties: { id: "hello", className: ["title", "main"] },
      children: [
        { type: "text", value: "Hello " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "world" }],
        },
      ],
    },
  ],
};

const html = hastToHtml(hast);

console.log(html);
// <h1 class="title main" id="hello">Hello <strong>world</strong></h1>
```

## 使用例

### 基本的なシリアライズ

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "This is " },
        {
          type: "element",
          tagName: "strong",
          properties: {},
          children: [{ type: "text", value: "bold" }],
        },
        { type: "text", value: " text." },
      ],
    },
  ],
};

console.log(hastToHtml(hast));
// <p>This is <strong>bold</strong> text.</p>
```

### void 要素

void 要素 (`<br>`、`<img>`、`<hr>` など) は自動的に自己閉じされます。

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "img",
      properties: { src: "photo.jpg", alt: "A photo" },
      children: [],
    },
  ],
};

console.log(hastToHtml(hast));
// <img alt="A photo" src="photo.jpg" />
```

### compile() の出力と組み合わせて使う

```ts
import { compile } from "@unifast/node";
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const result = compile("**bold text**", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

console.log(hastToHtml(hast));
// <p><strong>bold text</strong></p>
```

### 生の HTML のパススルー

```ts
import { hastToHtml } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const hast: HastRoot = {
  type: "root",
  children: [
    { type: "raw", value: "<div class=\"custom\">Raw HTML</div>" },
  ],
};

console.log(hastToHtml(hast));
// <div class="custom">Raw HTML</div>
```

## 動作

- **HTML エスケープ:** テキストコンテンツはエスケープされる (`&`、`<`、`>`、`"`)
- **void 要素:** `area`、`base`、`br`、`col`、`embed`、`hr`、`img`、`input`、`link`、`meta`、`param`、`source`、`track`、`wbr` は自己閉じされる
- **属性:** アルファベット順にソートされる。`className` 配列はスペースで結合されて `class` として出力される。真偽値 `true` は属性名のみで出力され、`false`/`null`/`undefined` は省略される
- **コメント:** `<!--value-->` としてレンダリングされる
- **doctype:** `<!DOCTYPE html>` としてレンダリングされる
- **raw ノード:** エスケープなしでそのまま出力される
