---
title: "accessibleEmoji()"
description: 'アクセシビリティのために、絵文字を aria-label 属性付きの <span role="img"> 要素でラップします。'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## シグネチャ

```ts
function accessibleEmoji(): UnifastPlugin
```

## パラメータ

なし。

## 使い方

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## 使用例

### aria ラベル付きで絵文字をラップする

各絵文字は、スクリーンリーダー向けに絵文字の意味を示す `aria-label` と `role="img"` を持つ `<span>` でラップされます。

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### 絵文字を含まないテキスト

絵文字を含まないプレーンテキストは、変更されずにそのまま出力されます。

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
