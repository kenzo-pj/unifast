---
title: "hastToReact()"
description: "Convert a HAST root node into React elements. The low-level function for when you need full control over the compilation pipeline."
---

```ts
import { hastToReact } from "@unifast/react";
```

### Signature

```ts
function hastToReact(
  hast: HastRoot,
  options: HastToReactOptions,
): unknown
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `hast` | `HastRoot` | A HAST root node |
| `options` | `HastToReactOptions` | React element creation config |

#### `HastToReactOptions`

| Property | Type | Description |
|----------|------|-------------|
| `createElement` | `CreateElement` | React's `createElement` function |
| `Fragment` | `unknown` | React's `Fragment` component |
| `components?` | `ComponentMap` | Map of HTML tag names to custom React components |

### Returns

`unknown` - A React element tree.

## Usage

### Basic usage

```tsx
import { createElement, Fragment } from "react";
import { compile } from "@unifast/node";
import { hastToReact } from "@unifast/react";
import type { HastRoot } from "@unifast/core";

// Step 1: Compile to HAST
const result = compile("# Hello, **world**!", { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

// Step 2: Convert to React elements
const element = hastToReact(hast, { createElement, Fragment });

function Page() {
  return <div>{element}</div>;
}
```

### With custom components

```tsx
import { createElement, Fragment } from "react";
import { compile } from "@unifast/node";
import { hastToReact } from "@unifast/react";
import type { HastRoot } from "@unifast/core";

const result = compile(md, { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

const element = hastToReact(hast, {
  createElement,
  Fragment,
  components: {
    pre: ({ children, ...props }) => (
      <pre className="code-block" {...props}>{children}</pre>
    ),
    a: ({ children, ...props }) => (
      <a target="_blank" rel="noopener" {...props}>{children}</a>
    ),
  },
});
```

### With Shiki-highlighted HAST

```tsx
import { createElement, Fragment } from "react";
import { compile } from "@unifast/node";
import { createShikiTransformer } from "@unifast/shiki";
import { hastToReact } from "@unifast/react";
import type { HastRoot } from "@unifast/core";

const transformer = await createShikiTransformer({
  themes: ["github-dark"],
  langs: ["typescript"],
});

const result = compile(md, { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);
const highlighted = transformer.transform(hast);

const element = hastToReact(highlighted, { createElement, Fragment });
```

### Server-side rendering

```tsx
import { createElement, Fragment } from "react";
import { renderToString } from "react-dom/server";
import { compile } from "@unifast/node";
import { hastToReact } from "@unifast/react";
import type { HastRoot } from "@unifast/core";

const result = compile(md, { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);
const element = hastToReact(hast, { createElement, Fragment });
const html = renderToString(element);
```

## Behavior

- **Property renaming:** HTML attributes are renamed to React equivalents (`class` to `className`, `for` to `htmlFor`, etc.)
- **Style parsing:** CSS style strings are parsed into React style objects
- **className arrays:** HAST `className` arrays are joined with spaces
- **Boolean attributes:** `true` renders the attribute, `false`/`null`/`undefined` omits it
- **Raw nodes:** Rendered as inline HTML - use the `sanitize` plugin (from `@unifast/node`) when processing untrusted input
- **Comments and doctype:** Ignored (return `null`)
