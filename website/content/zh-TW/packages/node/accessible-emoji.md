---
title: "accessibleEmoji()"
description: '將表情符號字元包進 <span role="img"> 元素中，並加上 aria-label 屬性以提升無障礙支援。'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## 函式簽名

```ts
function accessibleEmoji(): UnifastPlugin
```

## 參數

無。

## 用法

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## 範例

### 為表情符號加上 aria label

每個表情符號字元都會被包進 `<span>` 中，並加上 `role="img"` 與 `aria-label`，讓螢幕閱讀器能正確描述該表情符號：

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### 未含表情符號的文字

不含表情符號字元的純文字會原樣輸出，不會被修改：

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
