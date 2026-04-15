---
title: "escapeHtml()"
description: "Escape karakter spesial HTML dalam sebuah string."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## Signature

```ts
function escapeHtml(str: string): string
```

## Parameter

### str

| Properti | Tipe | Default | Deskripsi |
|----------|------|---------|-------------|
| `str` | `string` | — | String yang berisi karakter untuk di-escape |

## Return

`string` — String input dengan `&`, `<`, `>`, dan `"` diganti dengan entitas HTML yang setara.

## Penggunaan

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Contoh

### Escape dasar

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Escape konten yang dihasilkan pengguna

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Membangun atribut HTML yang aman

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Perilaku

- **`&`** diganti dengan `&amp;`
- **`<`** diganti dengan `&lt;`
- **`>`** diganti dengan `&gt;`
- **`"`** diganti dengan `&quot;`
- Semua karakter lain dibiarkan tidak berubah
