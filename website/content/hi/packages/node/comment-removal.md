---
title: "commentRemoval()"
description: "output से HTML comments हटाएँ।"
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Signature

```ts
function commentRemoval(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// HTML comment output से हटा दिया जाता है
```

## उदाहरण

### मूल comment हटाना

सभी HTML comment nodes (`<!-- ... -->`) को output tree से हटा दिया जाता है, जिसमें blockquotes जैसे block elements के अंदर nested comments भी शामिल हैं:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### non-comment HTML बरकरार रहता है

केवल comment nodes हटाए जाते हैं। अन्य inline HTML अछूता रहता है:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
