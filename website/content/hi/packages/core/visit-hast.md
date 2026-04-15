---
title: "visitHast()"
description: "एक visitor function का उपयोग करके HAST tree को walk और transform करें।"
---

```ts
import { visitHast } from "@unifast/core";
```

## Signature

```ts
function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode
```

## Parameters

### node

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `type` | `string` | — | node type (`"root"`, `"element"`, `"text"`, आदि) |
| `children` | `HastNode[]` | — | child nodes (`"root"` और `"element"` types के लिए) |

### visitor

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `visitor` | `(node: HastNode) => HastNode \| void` | — | प्रत्येक node के लिए call किया जाने वाला एक function; मूल node को बदलने के लिए एक node return करें, या उसे बनाए रखने के लिए `void` |

## Returns

`HastNode` — एक नया tree जिसमें visitor function द्वारा लागू transformations हैं।

## उपयोग

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Hello world" }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "text") {
    return { type: "text", value: node.value.toUpperCase() };
  }
});

console.log(result);
// text "HELLO WORLD" के साथ tree
```

## उदाहरण

### सभी paragraphs में एक class जोड़ें

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "First paragraph." }],
    },
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [{ type: "text", value: "Second paragraph." }],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "p") {
    return {
      ...node,
      properties: { ...node.properties, className: ["prose"] },
    };
  }
});

// दोनों <p> elements में अब className: ["prose"] है
```

### सभी images हटाएँ

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "See the photo: " },
        {
          type: "element",
          tagName: "img",
          properties: { src: "photo.jpg", alt: "A photo" },
          children: [],
        },
      ],
    },
  ],
};

const result = visitHast(tree, (node) => {
  if (node.type === "element" && (node.tagName === "p" || node.tagName === "div")) {
    return {
      ...node,
      children: node.children.filter(
        (child) => !(child.type === "element" && child.tagName === "img"),
      ),
    };
  }
});

// <img> element को tree से हटा दिया गया है
```

### सभी links एकत्र करें

```ts
import { visitHast } from "@unifast/core";
import type { HastRoot } from "@unifast/core";

const tree: HastRoot = {
  type: "root",
  children: [
    {
      type: "element",
      tagName: "p",
      properties: {},
      children: [
        { type: "text", value: "Visit " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://example.com" },
          children: [{ type: "text", value: "Example" }],
        },
        { type: "text", value: " and " },
        {
          type: "element",
          tagName: "a",
          properties: { href: "https://docs.example.com" },
          children: [{ type: "text", value: "Docs" }],
        },
      ],
    },
  ],
};

const links: string[] = [];

visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "a") {
    const href = node.properties.href;
    if (typeof href === "string") {
      links.push(href);
    }
  }
});

console.log(links);
// ["https://example.com", "https://docs.example.com"]
```

### Code blocks को एक container में wrap करें

```ts
import { visitHast } from "@unifast/core";
import type { HastNode, HastRoot } from "@unifast/core";

const tree: HastRoot = {
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

const result = visitHast(tree, (node) => {
  if (node.type === "element" && node.tagName === "pre") {
    return {
      type: "element",
      tagName: "div",
      properties: { className: ["code-block"] },
      children: [node],
    } as HastNode;
  }
});

// <pre> अब <div class="code-block"> के अंदर wrap हो गया है
```

## व्यवहार

- **Immutable:** एक नया tree return करता है; मूल tree modified नहीं होता
- **Top-down traversal:** visitor को parent पर उसके children को visit करने से पहले call किया जाता है
- **Replacement:** यदि visitor कोई node return करता है, तो वह children को traverse करने से पहले current node को replace कर देता है
- **No-op:** यदि visitor `void` (या `undefined`) return करता है, तो मूल node बरकरार रखा जाता है
- **Recursive:** `"root"` और `"element"` nodes के children को recursively visit किया जाता है
