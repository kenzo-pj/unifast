---
title: "customHeadingId()"
description: "{#custom-id} syntax का उपयोग करके headings पर custom IDs set करें।"
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Signature

```ts
function customHeadingId(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// auto-generated slug के बजाय heading को id="intro" मिलता है
```

## उदाहरण

### Custom ID

एक विशिष्ट `id` attribute assign करने के लिए heading के अंत में `{#custom-id}` syntax का उपयोग करें। brace block को rendered text से हटा दिया जाता है:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### custom ID के बिना Heading

`{#...}` syntax के बिना headings अपरिवर्तित रहती हैं। वे heading text से उत्पन्न default slug का उपयोग करती हैं:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Classes और arbitrary attributes

brace syntax `.class` और `key=value` notation को भी सपोर्ट करता है:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
