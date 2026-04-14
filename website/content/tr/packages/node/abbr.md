---
title: "abbr()"
description: "Kısaltma tanımlarını title öznitelikli <abbr> elemanlarına dönüştürür."
---

```ts
import { abbr } from "@unifast/node";
```

## İmza

```ts
function abbr(): UnifastPlugin
```

## Parametreler

Yok.

## Kullanım

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// "HTML" geçişleri <abbr title="Hyper Text Markup Language"> ile sarmalanır
```

## Örnekler

### Temel kısaltma

Bir kısaltmayı `*[TERM]: Definition` sözdizimiyle tanımlayın. Tanım paragrafı çıktıdan kaldırılır ve terimin tüm geçişleri `<abbr>` elemanlarıyla sarmalanır:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### Birden fazla kısaltma

Birden fazla kısaltma tanımlayabilirsiniz. Her terim belge genelinde bağımsız olarak değiştirilir:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
