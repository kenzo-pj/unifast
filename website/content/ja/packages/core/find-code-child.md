---
title: "findCodeChild()"
description: "親要素の内部から子の <code> 要素を見つけます。"
---

```ts
import { findCodeChild } from "@unifast/core";
```

## シグネチャ

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## パラメータ

### element

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `type` | `"element"` | — | ノード種別の識別子 |
| `tagName` | `string` | — | HTML のタグ名 (通常は `"pre"`) |
| `properties` | `Record<string, unknown>` | — | 要素のプロパティ |
| `children` | `HastNode[]` | — | 検索対象となる子ノード |

## 戻り値

`HastElement | undefined` — `tagName` が `"code"` である最初の子要素です。該当する子が存在しない場合は `undefined` を返します。

## 使い方

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-js"] },
      children: [{ type: "text", value: "const x = 1;" }],
    },
  ],
};

const code = findCodeChild(pre);

console.log(code?.tagName);
// code
```

## 使用例

### pre 要素の内部から code を見つける

```ts
import { findCodeChild, extractLang, extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-rust"] },
      children: [{ type: "text", value: 'fn main() { println!("hello"); }' }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // rust
  console.log(extractText(code));
  // fn main() { println!("hello"); }
}
```

### code の子が存在しない場合

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    { type: "text", value: "plain preformatted text" },
  ],
};

const code = findCodeChild(pre);

console.log(code);
// undefined
```

### シンタックスハイライトのために visitHast と組み合わせて使う

```ts
import { visitHast, findCodeChild, extractLang } from "@unifast/core";
import type { HastNode, HastElement } from "@unifast/core";

const tree: HastNode = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    const code = findCodeChild(node);
    if (code) {
      const lang = extractLang(code);
      console.log(`Found code block with language: ${lang}`);
      // Found code block with language: js
    }
  }
});
```
