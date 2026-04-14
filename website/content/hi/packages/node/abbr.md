---
title: "abbr()"
description: "abbreviation definitions को title attributes के साथ <abbr> elements में convert करें।"
---

```ts
import { abbr } from "@unifast/node";
```

## Signature

```ts
function abbr(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

The HTML specification is maintained by the W3C.`;

const result = compile(md, {
  plugins: [abbr()],
});
// "HTML" की घटनाएँ <abbr title="Hyper Text Markup Language"> में wrap हो जाती हैं
```

## उदाहरण

### मूल abbreviation

`*[TERM]: Definition` syntax के साथ एक abbreviation define करें। definition paragraph output से हटा दिया जाता है, और term की सभी घटनाएँ `<abbr>` elements में wrap हो जाती हैं:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language

HTML is the standard markup language for web pages.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> is the standard markup language for web pages.</p>
```

### कई abbreviations

आप कई abbreviations define कर सकते हैं। प्रत्येक term को document में स्वतंत्र रूप से replace किया जाता है:

```ts
import { compile, abbr } from "@unifast/node";

const md = `*[HTML]: Hyper Text Markup Language
*[CSS]: Cascading Style Sheets

HTML and CSS are the foundations of the web.`;

const result = compile(md, { plugins: [abbr()] });
console.log(result.output);
// <p><abbr title="Hyper Text Markup Language">HTML</abbr> and <abbr title="Cascading Style Sheets">CSS</abbr> are the foundations of the web.</p>
```
