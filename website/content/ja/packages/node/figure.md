---
title: "figure()"
description: "alt テキストを持つ画像を `<figure>` と `<figcaption>` 要素でラップします。"
---

```ts
import { figure } from "@unifast/node";
```

## シグネチャ

```ts
function figure(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## 使用例

### 基本的な figure ラップ

画像に alt テキストがある場合、`figure()` はそれを `<figure>` 要素でラップし、alt テキストを含む `<figcaption>` を追加します。

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### alt テキストのない画像

alt テキストを持たない画像は、表示するキャプションがないためラップされません。

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
