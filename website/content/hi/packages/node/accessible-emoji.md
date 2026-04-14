---
title: "accessibleEmoji()"
description: 'accessibility के लिए emoji characters को aria-label attributes के साथ <span role="img"> elements में wrap करें।'
---

```ts
import { accessibleEmoji } from "@unifast/node";
```

## Signature

```ts
function accessibleEmoji(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Launch day! \u{1F680}`;

const result = compile(md, {
  plugins: [accessibleEmoji()],
});
// Emoji <span role="img" aria-label="rocket"> में wrap हो जाता है
```

## उदाहरण

### aria labels के साथ emoji wrapping

प्रत्येक emoji character को `role="img"` और screen readers के लिए emoji का वर्णन करने वाले `aria-label` के साथ एक `<span>` में wrap किया जाता है:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `Great job \u{1F44D} keep it up \u{1F525}!`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>Great job <span aria-label="thumbs up" role="img">\u{1F44D}</span> keep it up <span aria-label="fire" role="img">\u{1F525}</span>!</p>
```

### emoji के बिना text

emoji characters के बिना plain text अपरिवर्तित pass हो जाता है:

```ts
import { compile, accessibleEmoji } from "@unifast/node";

const md = `No emoji here, just plain text.`;

const result = compile(md, { plugins: [accessibleEmoji()] });
console.log(result.output);
// <p>No emoji here, just plain text.</p>
```
