---
title: "toc()"
description: "Create a TOC plugin that extracts headings from Markdown/MDX and populates CompileResult.toc."
---

```ts
import { toc } from "@unifast/plugin-toc";
```

### Signature

```ts
function toc(options?: TocPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `TocPluginOptions` | TOC extraction configuration |

#### `TocPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `maxDepth` | `number` | `3` | Maximum heading depth to include (1-6). A value of `3` includes h1, h2, and h3. |

### Returns

`UnifastPlugin`

## Usage

### Basic usage

```ts
import { compile } from "@unifast/node";
import { toc } from "@unifast/plugin-toc";

const md = `
# Introduction

## Getting Started

### Installation

## API Reference

### Functions

### Types
`;

const result = compile(md, { plugins: [toc()] });

console.log(result.toc);
// [
//   { depth: 1, text: "Introduction", slug: "introduction" },
//   { depth: 2, text: "Getting Started", slug: "getting-started" },
//   { depth: 3, text: "Installation", slug: "installation" },
//   { depth: 2, text: "API Reference", slug: "api-reference" },
//   { depth: 3, text: "Functions", slug: "functions" },
//   { depth: 3, text: "Types", slug: "types" },
// ]
```

### Limit depth

```ts
import { compile } from "@unifast/node";
import { toc } from "@unifast/plugin-toc";

// Only include h1 and h2 headings
const result = compile(md, {
  plugins: [toc({ maxDepth: 2 })],
});

console.log(result.toc);
// [
//   { depth: 1, text: "Introduction", slug: "introduction" },
//   { depth: 2, text: "Getting Started", slug: "getting-started" },
//   { depth: 2, text: "API Reference", slug: "api-reference" },
// ]
```

### Render a TOC sidebar

```ts
import { compile } from "@unifast/node";
import { toc } from "@unifast/plugin-toc";

const result = compile(md, { plugins: [toc({ maxDepth: 3 })] });

function renderToc(entries: typeof result.toc): string {
  return `<nav class="toc"><ul>${entries
    .map(
      (entry) =>
        `<li class="toc-depth-${entry.depth}"><a href="#${entry.slug}">${entry.text}</a></li>`
    )
    .join("")}</ul></nav>`;
}

const tocHtml = renderToc(result.toc);
```

### Build a nested tree

```ts
import { compile } from "@unifast/node";
import { toc } from "@unifast/plugin-toc";
import type { TocEntry } from "@unifast/core";

type TocTree = TocEntry & { children: TocTree[] };

function buildTocTree(entries: TocEntry[]): TocTree[] {
  const root: TocTree[] = [];
  const stack: TocTree[] = [];

  for (const entry of entries) {
    const node: TocTree = { ...entry, children: [] };
    while (stack.length > 0 && stack[stack.length - 1].depth >= entry.depth) {
      stack.pop();
    }
    if (stack.length === 0) {
      root.push(node);
    } else {
      stack[stack.length - 1].children.push(node);
    }
    stack.push(node);
  }

  return root;
}

const result = compile(md, { plugins: [toc()] });
const tree = buildTocTree(result.toc);
```

### Combined with other plugins

```ts
import { compile } from "@unifast/node";
import { toc } from "@unifast/plugin-toc";
import { gfm } from "@unifast/plugin-gfm";
import { frontmatter } from "@unifast/plugin-frontmatter";

const result = compile(md, {
  plugins: [gfm(), frontmatter(), toc({ maxDepth: 3 })],
});

console.log(result.frontmatter);
console.log(result.toc);
console.log(result.output);
```
