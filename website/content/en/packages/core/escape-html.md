---
title: "escapeHtml()"
description: "Escape HTML special characters in a string."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Signature

```ts
function escapeHtml(str: string): string
```

## Parameters

### str

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `str` | `string` | — | The string containing characters to escape |

## Returns

`string` — The input string with `&`, `<`, `>`, and `"` replaced by their HTML entity equivalents.

## Usage

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Examples

### Basic escaping

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Escaping user-generated content

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Building safe HTML attributes

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Behavior

- **`&`** is replaced with `&amp;`
- **`<`** is replaced with `&lt;`
- **`>`** is replaced with `&gt;`
- **`"`** is replaced with `&quot;`
- All other characters are left unchanged
