---
title: "commentRemoval()"
description: "出力から HTML コメントを削除します。"
---

```ts
import { commentRemoval } from "@unifast/node";
```

## シグネチャ

```ts
function commentRemoval(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## 使用例

### 基本的なコメント削除

引用などのブロック要素の内部にネストされたコメントも含めて、すべての HTML コメントノード (`<!-- ... -->`) が出力ツリーから除去されます。

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### コメント以外の HTML は保持される

削除されるのはコメントノードのみです。その他のインライン HTML は手つかずのまま残ります。

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
