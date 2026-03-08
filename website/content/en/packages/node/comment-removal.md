---
title: "commentRemoval()"
description: "Remove HTML comments from the output."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Signature

```ts
function commentRemoval(): UnifastPlugin
```

## Parameters

None.

## Usage

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## Examples

### Basic comment removal

All HTML comment nodes (`<!-- ... -->`) are stripped from the output tree, including comments nested inside block elements like blockquotes:

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

### Non-comment HTML is preserved

Only comment nodes are removed. Other inline HTML is left untouched:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
