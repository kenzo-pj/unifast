---
title: "minify()"
description: "不要な空白を除去して HTML 出力を最小化します。"
---

```ts
import { minify } from "@unifast/node";
```

## シグネチャ

```ts
function minify(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## 使用例

### 基本的な最小化

`minify()` プラグインは、連続する空白文字を 1 つのスペースに圧縮し、HTML コメントを削除します。さらにブロック要素間の空白のみのテキストノードを除去し、空の `class` 属性と `style` 属性も削除します。

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### 整形済みのコンテンツは保持される

`<pre>` や `<code>` ブロック内の空白には手を加えないため、コードの整形が崩れることはありません。

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
