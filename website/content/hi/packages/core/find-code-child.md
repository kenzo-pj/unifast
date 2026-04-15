---
title: "findCodeChild()"
description: "एक parent element के अंदर child <code> element खोजें।"
---

```ts
import { findCodeChild } from "@unifast/core";
```

## Signature

```ts
function findCodeChild(element: HastElement): HastElement | undefined
```

## Parameters

### element

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `type` | `"element"` | — | Node type identifier |
| `tagName` | `string` | — | HTML tag name (आमतौर पर `"pre"`) |
| `properties` | `Record<string, unknown>` | — | Element properties |
| `children` | `HastNode[]` | — | खोजने के लिए child nodes |

## Returns

`HastElement | undefined` — `"code"` के `tagName` वाला पहला child element, या यदि ऐसा कोई child मौजूद नहीं है तो `undefined`।

## उपयोग

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-js"] },
      children: [{ type: "text", value: "const x = 1;" }],
    },
  ],
};

const code = findCodeChild(pre);

console.log(code?.tagName);
// code
```

## उदाहरण

### एक pre element के अंदर code खोजें

```ts
import { findCodeChild, extractLang, extractText } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    {
      type: "element",
      tagName: "code",
      properties: { className: ["language-rust"] },
      children: [{ type: "text", value: 'fn main() { println!("hello"); }' }],
    },
  ],
};

const code = findCodeChild(pre);
if (code) {
  console.log(extractLang(code));
  // rust
  console.log(extractText(code));
  // fn main() { println!("hello"); }
}
```

### जब कोई code child मौजूद नहीं है

```ts
import { findCodeChild } from "@unifast/core";
import type { HastElement } from "@unifast/core";

const pre: HastElement = {
  type: "element",
  tagName: "pre",
  properties: {},
  children: [
    { type: "text", value: "plain preformatted text" },
  ],
};

const code = findCodeChild(pre);

console.log(code);
// undefined
```

### Syntax highlighting के लिए visitHast के साथ उपयोग

```ts
import { visitHast, findCodeChild, extractLang } from "@unifast/core";
import type { HastNode, HastElement } from "@unifast/core";

const tree: HastNode = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "pre",
      properties: {},
      children: [
        {
          type: "element",
          tagName: "code",
          properties: { className: ["language-js"] },
          children: [{ type: "text", value: "const x = 1;" }],
        },
      ],
    },
  ],
};

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    const code = findCodeChild(node);
    if (code) {
      const lang = extractLang(code);
      console.log(`Found code block with language: ${lang}`);
      // Found code block with language: js
    }
  }
});
```
