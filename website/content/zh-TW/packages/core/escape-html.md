---
title: "escapeHtml()"
description: "跳脫字串中的 HTML 特殊字元。"
---

```ts
import { escapeHtml } from "@unifast/core";
```

## 函式簽名

```ts
function escapeHtml(str: string): string
```

## 參數

### str

| 屬性 | 型別 | 預設值 | 說明 |
|----------|------|---------|-------------|
| `str` | `string` | — | 包含要跳脫字元的字串 |

## 回傳值

`string` — 已將 `&`、`<`、`>` 與 `"` 取代為對應 HTML 實體的字串。

## 用法

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## 範例

### 基本跳脫

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### 跳脫使用者產生的內容

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### 建構安全的 HTML 屬性

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## 行為

- **`&`** 會被取代為 `&amp;`
- **`<`** 會被取代為 `&lt;`
- **`>`** 會被取代為 `&gt;`
- **`"`** 會被取代為 `&quot;`
- 其他所有字元都會保持原樣
