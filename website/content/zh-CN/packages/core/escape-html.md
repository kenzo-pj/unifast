---
title: "escapeHtml()"
description: "转义字符串中的 HTML 特殊字符。"
---

```ts
import { escapeHtml } from "@unifast/core";
```

## 签名

```ts
function escapeHtml(str: string): string
```

## 参数

### str

| 属性 | 类型 | 默认值 | 描述 |
|----------|------|---------|-------------|
| `str` | `string` | — | 包含需要转义字符的字符串 |

## 返回值

`string` —— 将 `&`、`<`、`>` 和 `"` 替换为相应 HTML 实体后的字符串。

## 用法

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## 示例

### 基础转义

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### 转义用户生成的内容

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### 构建安全的 HTML 属性

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## 行为说明

- **`&`** 会被替换为 `&amp;`
- **`<`** 会被替换为 `&lt;`
- **`>`** 会被替换为 `&gt;`
- **`"`** 会被替换为 `&quot;`
- 其他字符保持不变
