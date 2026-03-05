---
title: "compileToReact()"
description: "Compile Markdown or MDX input and return React elements directly, along with frontmatter, diagnostics, stats, and TOC."
---

```ts
import { compileToReact } from "@unifast/react";
```

### Signature

```ts
function compileToReact(
  input: string,
  options: CompileToReactOptions,
): CompileToReactResult
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `string` | Markdown or MDX source |
| `options` | `CompileToReactOptions` | Compile options plus React-specific config |

#### `CompileToReactOptions`

Extends [`CompileOptions`](/docs/packages/core/overview) with React-specific fields.

| Property | Type | Description |
|----------|------|-------------|
| `createElement` | `CreateElement` | React's `createElement` function |
| `Fragment` | `unknown` | React's `Fragment` component |
| `components?` | `ComponentMap` | Map of HTML tag names to React components |
| *(all other fields)* | - | Inherited from [`CompileOptions`](/docs/packages/core/overview) |

### Returns

`CompileToReactResult`

| Property | Type | Description |
|----------|------|-------------|
| `element` | `unknown` | The root React element |
| `frontmatter` | `Record<string, unknown>` | Parsed frontmatter metadata |
| `diagnostics` | `Diagnostic[]` | Warnings and errors |
| `stats` | `{ parseMs, transformMs, emitMs }` | Timing breakdown (ms) |
| `toc` | `TocEntry[]` | Extracted table of contents |

## Usage

### Basic usage

```tsx
import { createElement, Fragment } from "react";
import { compileToReact } from "@unifast/react";

const { element, frontmatter, toc } = compileToReact(
  "# Hello, **world**!",
  { createElement, Fragment }
);

function Page() {
  return <div>{element}</div>;
}
```

### With custom components

```tsx
import { createElement, Fragment } from "react";
import { compileToReact } from "@unifast/react";

const components = {
  h1: ({ children, ...props }) => (
    <h1 className="text-4xl font-bold" {...props}>{children}</h1>
  ),
  a: ({ children, ...props }) => (
    <a className="text-blue-500 underline" target="_blank" {...props}>{children}</a>
  ),
  code: ({ children, ...props }) => (
    <code className="bg-gray-100 rounded px-1" {...props}>{children}</code>
  ),
};

const { element } = compileToReact(md, {
  createElement,
  Fragment,
  components,
});
```

### With plugins

```tsx
import { createElement, Fragment } from "react";
import { compileToReact } from "@unifast/react";
import { gfm, frontmatter, toc } from "@unifast/node";

const result = compileToReact(md, {
  createElement,
  Fragment,
  plugins: [gfm(), frontmatter(), toc()],
});

console.log(result.frontmatter);
console.log(result.toc);
```

### Server-side rendering

```tsx
import { createElement, Fragment } from "react";
import { renderToString } from "react-dom/server";
import { compileToReact } from "@unifast/react";

const { element } = compileToReact(md, { createElement, Fragment });
const html = renderToString(element);
```

### Using frontmatter and TOC

```tsx
import { createElement, Fragment } from "react";
import { compileToReact } from "@unifast/react";
import { frontmatter, toc } from "@unifast/node";

const md = `---
title: My Page
---

# Introduction

## Setup

## Usage
`;

const result = compileToReact(md, {
  createElement,
  Fragment,
  plugins: [frontmatter(), toc()],
});

function Page() {
  return (
    <div>
      <h1>{result.frontmatter.title as string}</h1>
      <nav>
        <ul>
          {result.toc.map((entry) => (
            <li key={entry.slug}>
              <a href={`#${entry.slug}`}>{entry.text}</a>
            </li>
          ))}
        </ul>
      </nav>
      <article>{result.element}</article>
    </div>
  );
}
```
