---
title: "accessibleEmoji()"
description: 'Bọc các ký tự emoji trong phần tử <span role="img"> kèm thuộc tính aria-label để hỗ trợ khả năng truy cập.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Chữ ký

```ts
function accessibleEmoji(): UnifastPlugin
```

## Tham số

Không có.

## Cách dùng

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Ví dụ

### Bọc emoji bằng aria label

Mỗi ký tự emoji được bọc trong một `<span>` với `role="img"` và một `aria-label` mô tả emoji đó để screen reader có thể đọc:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Văn bản không có emoji

Văn bản thuần không chứa ký tự emoji được đi qua không thay đổi:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
