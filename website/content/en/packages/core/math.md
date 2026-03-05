---
title: "math()"
description: "Enable math expressions with inline and display syntax."
---

```ts
import { math } from "@unifast/node";
```

### Signature

```ts
function math(): UnifastPlugin
```

### Parameters

None.

### Returns

`UnifastPlugin`

## Usage

### Basic usage

```ts
import { compile, math } from "@unifast/node";

const md = `
Einstein's famous equation: $E = mc^2$

The sum of integers from 1 to n:

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$
`;

const result = compile(md, { plugins: [math()] });
```

### Inline math

```ts
import { compile, math } from "@unifast/node";

const md = `
The quadratic formula is $x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}$ and is used to solve quadratic equations.
`;

const result = compile(md, { plugins: [math()] });
```

### Display math

```ts
import { compile, math } from "@unifast/node";

const md = `
$$
\\int_{0}^{\\infty} e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}
$$
`;

const result = compile(md, { plugins: [math()] });
```

### Including KaTeX CSS

The plugin converts math syntax into the appropriate HTML structure, but you need to include KaTeX CSS separately for proper rendering in the browser.

```html
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"
  crossorigin="anonymous"
/>
```

### Combined with other plugins

```ts
import { compile, math, gfm, frontmatter } from "@unifast/node";

const result = compile(md, {
  plugins: [math(), gfm(), frontmatter()],
});
```
