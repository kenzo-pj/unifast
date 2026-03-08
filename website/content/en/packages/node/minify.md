---
title: "minify()"
description: "Minify the HTML output by removing unnecessary whitespace."
---

```ts
import { minify } from "@unifast/node";
```

## Signature

```ts
function minify(): UnifastPlugin
```

## Parameters

None.

## Usage

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Examples

### Basic minification

The `minify()` plugin collapses consecutive whitespace characters into single spaces, removes HTML comments, strips whitespace-only text nodes between block elements, and removes empty `class` and `style` attributes:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Preformatted content is preserved

Whitespace inside `<pre>` and `<code>` blocks is left untouched, so code formatting is never broken:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
