---
title: "accessibleEmoji()"
description: '为提升可访问性，将 emoji 字符包裹在带有 aria-label 的 <span role="img"> 元素中。'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## 签名

```ts
function accessibleEmoji(): UnifastPlugin
```

## 参数

无。

## 用法

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## 示例

### 为 emoji 添加 aria 标签

每个 emoji 字符都会被包裹在带有 `role="img"` 和 `aria-label` 的 `<span>` 中，aria-label 会对 emoji 进行描述，以便屏幕阅读器朗读：

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### 不含 emoji 的文本

不含 emoji 字符的纯文本会原样输出：

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
