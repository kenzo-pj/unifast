---
title: "figure()"
description: "Wrap images that have alt text in <figure> and <figcaption> elements."
---

```ts
import { figure } from "@unifast/node";
```

## Signature

```ts
function figure(): UnifastPlugin
```

## Parameters

None.

## Usage

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// The image is wrapped in <figure> with <figcaption>
```

## Examples

### Basic figure wrapping

When an image has alt text, `figure()` wraps it in a `<figure>` element and adds a `<figcaption>` containing the alt text:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### Image without alt text

Images without alt text are not wrapped, since there is no meaningful caption to display:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
