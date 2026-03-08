---
title: "accessibleEmoji()"
description: 'Wrap emoji characters in <span role="img"> elements with aria-label attributes for accessibility.'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Signature

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parameters

None.

## Usage

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji is wrapped in <span role="img" aria-label="rocket">
```

## Examples

### Emoji wrapping with aria labels

Each emoji character is wrapped in a `<span>` with `role="img"` and an `aria-label` describing the emoji for screen readers:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### Text without emoji

Plain text without emoji characters passes through unchanged:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
