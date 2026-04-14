---
title: "escapeHtml()"
description: "एक string में HTML special characters को escape करें।"
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

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `str` | `string` | — | वह string जिसमें escape किए जाने वाले characters हैं |

## Returns

`string` — input string जिसमें `&`, `<`, `>`, और `"` को उनके HTML entity समकक्षों से बदल दिया गया है।

## उपयोग

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## उदाहरण

### मूल escaping

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### User-generated content को escape करना

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### सुरक्षित HTML attributes बनाना

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## व्यवहार

- **`&`** को `&amp;` से बदला जाता है
- **`<`** को `&lt;` से बदला जाता है
- **`>`** को `&gt;` से बदला जाता है
- **`"`** को `&quot;` से बदला जाता है
- अन्य सभी characters अपरिवर्तित रहते हैं
