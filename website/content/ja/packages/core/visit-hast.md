---
title: "visitHast()"
description: "ビジター関数を使って HAST ツリーを走査・変換します。"
---

```ts
import { visitHast } from "@unifast/core";
```

## シグネチャ

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## パラメータ

### node

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `type` | `string` | — | ノードの種別 (`"root"`、`"element"`、`"text"` など) |
| `children` | `HastNode[]` | — | 子ノード (`"root"` と `"element"` 型の場合) |

### visitor

| プロパティ | 型 | デフォルト | 説明 |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | 各ノードに対して呼ばれる関数。元のノードを置き換えたい場合はノードを返し、そのままにしたい場合は `void` を返します |

## 戻り値

`HastNode` — ビジター関数による変換が適用された新しいツリーです。

## 使い方

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// Tree with text "HELLO WORLD"
```

## 使用例

### すべての段落にクラスを追加する

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// Both <p> elements now have className: ["prose"]
```

### すべての画像を削除する

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// The <img> element has been removed from the tree
```

### すべてのリンクを収集する

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### コードブロックをコンテナでラップする

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> is now wrapped inside <div class="code-block">
```

## 動作

- **イミュータブル:** 新しいツリーを返し、元のツリーは変更しない
- **トップダウンの走査:** 子ノードを訪問する前に、親ノードに対してビジターが呼ばれる
- **置き換え:** ビジターがノードを返した場合、子ノードを走査する前に現在のノードが置き換えられる
- **ノーオペレーション:** ビジターが `void` (または `undefined`) を返した場合、元のノードがそのまま残る
- **再帰的:** `"root"` と `"element"` ノードの子は再帰的に訪問される
