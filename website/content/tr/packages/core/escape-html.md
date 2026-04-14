---
title: "escapeHtml()"
description: "Bir string içindeki HTML özel karakterlerini kaçış karakterine dönüştürür."
---

```ts
import { escapeHtml } from "@unifast/core";
```

## İmza

```ts
function escapeHtml(str: string): string
```

## Parametreler

### str

| Özellik | Tür | Varsayılan | Açıklama |
|---------|-----|------------|----------|
| `str` | `string` | — | Kaçış karakterine dönüştürülecek karakterleri içeren string |

## Dönüş Değeri

`string` — `&`, `<`, `>` ve `"` karakterleri HTML varlık karşılıklarıyla değiştirilmiş girdi string'i.

## Kullanım

```ts
import { escapeHtml } from "@unifast/core";

const safe = escapeHtml('<script>alert("xss")</script>');

console.log(safe);
// &lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;
```

## Örnekler

### Temel kaçış

```ts
import { escapeHtml } from "@unifast/core";

console.log(escapeHtml("Tom & Jerry"));
// Tom &amp; Jerry

console.log(escapeHtml('class="main"'));
// class=&quot;main&quot;

console.log(escapeHtml("1 < 2 > 0"));
// 1 &lt; 2 &gt; 0
```

### Kullanıcı tarafından oluşturulan içerikte kaçış

```ts
import { escapeHtml } from "@unifast/core";

const userComment = '<img src=x onerror="alert(1)">';
const html = `<div class="comment">${escapeHtml(userComment)}</div>`;

console.log(html);
// <div class="comment">&lt;img src=x onerror=&quot;alert(1)&quot;&gt;</div>
```

### Güvenli HTML öznitelikleri oluşturma

```ts
import { escapeHtml } from "@unifast/core";

const title = 'He said "hello" & waved';
const html = `<span title="${escapeHtml(title)}">Hover me</span>`;

console.log(html);
// <span title="He said &quot;hello&quot; &amp; waved">Hover me</span>
```

## Davranış

- **`&`**, `&amp;` ile değiştirilir
- **`<`**, `&lt;` ile değiştirilir
- **`>`**, `&gt;` ile değiştirilir
- **`"`**, `&quot;` ile değiştirilir
- Diğer tüm karakterler değiştirilmeden bırakılır
