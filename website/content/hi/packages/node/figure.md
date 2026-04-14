---
title: "figure()"
description: "alt text वाली images को <figure> और <figcaption> elements में wrap करें।"
---

```ts
import { figure } from "@unifast/node";
```

## Signature

```ts
function figure(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, figure } from "@unifast/node";

const md = `![A sunset over the ocean](sunset.jpg)`;

const result = compile(md, {
  plugins: [figure()],
});
// Image <figcaption> के साथ <figure> में wrap हो जाता है
```

## उदाहरण

### मूल figure wrapping

जब किसी image में alt text होता है, तो `figure()` इसे एक `<figure>` element में wrap करता है और alt text वाला एक `<figcaption>` जोड़ता है:

```ts
import { compile, figure } from "@unifast/node";

const md = `![A beautiful landscape](landscape.jpg)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <figure><img src="landscape.jpg" alt="A beautiful landscape"><figcaption>A beautiful landscape</figcaption></figure>
```

### alt text के बिना Image

alt text के बिना images wrap नहीं होतीं, क्योंकि display करने के लिए कोई meaningful caption नहीं होता:

```ts
import { compile, figure } from "@unifast/node";

const md = `![](decorative.png)`;

const result = compile(md, { plugins: [figure()] });
console.log(result.output);
// <p><img src="decorative.png" alt=""></p>
```
