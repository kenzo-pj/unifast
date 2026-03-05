---
title: "compile()"
description: "The primary compilation function. Transforms Markdown or MDX into HTML or other output formats using the native Rust compiler."
---

```ts
import { compile } from "@unifast/node";
```

### Signature

```ts
function compile(input: string, options?: CompileOptions): CompileResult
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `string` | Markdown or MDX source string |
| `options?` | `CompileOptions` | Compilation configuration including plugins |

### Returns

| Property | Type | Description |
|----------|------|-------------|
| `output` | `string \| object` | Compiled output. HTML string by default; JSON string for HAST/MDAST. |
| `frontmatter` | `Record<string, unknown>` | Parsed frontmatter metadata (empty `{}` if none) |
| `diagnostics` | `Diagnostic[]` | Array of `{ level, message, start?, end? }` |
| `stats` | `{ parseMs, transformMs, emitMs }` | Timing breakdown (ms) |
| `toc` | `TocEntry[]` | Extracted table of contents (empty `[]` if TOC disabled) |

## Usage

### Basic Markdown to HTML

```ts
import { compile } from "@unifast/node";

const result = compile("# Hello, **world**!");
console.log(result.output);
// <h1 id="hello-world">Hello, <strong>world</strong>!</h1>
```

### MDX input

```ts
const mdx = `
import { Alert } from "./components";

# Hello

<Alert type="info">This is MDX!</Alert>
`;

const result = compile(mdx, { inputKind: "mdx" });
```

### HAST output

```ts
const result = compile("# Hello", { outputKind: "hast" });
const hast = JSON.parse(result.output as string);
// { type: "root", children: [{ type: "element", tagName: "h1", ... }] }
```

### MDAST output

```ts
const result = compile("# Hello", { outputKind: "mdast" });
const mdast = JSON.parse(result.output as string);
```

### MDX JS output

```ts
const result = compile(mdxSource, {
  inputKind: "mdx",
  outputKind: "mdxJs",
});
```

### With GFM

```ts
const result = compile(md, {
  gfm: {
    tables: true,
    taskList: true,
    strikethrough: true,
    footnotes: true,
    autolink: true,
  },
});
```

### With frontmatter

```ts
const md = `---
title: My Doc
---

# Content
`;

const result = compile(md, {
  frontmatter: { yaml: true },
});
console.log(result.frontmatter.title); // "My Doc"
```

### With sanitization

```ts
const result = compile(untrustedInput, {
  sanitize: {
    enabled: true,
    schema: {
      allowedTags: ["p", "a", "strong", "em", "h1", "h2", "h3"],
      allowedAttributes: { a: ["href"] },
      allowedProtocols: { href: ["https", "mailto"] },
    },
  },
});
```

### With syntax highlighting

```ts
const result = compile(md, {
  highlight: { enabled: true, engine: "syntect" },
});
```

### With TOC extraction

```ts
const result = compile(md, {
  toc: { enabled: true, maxDepth: 3 },
});
console.log(result.toc);
// [{ depth: 1, text: "...", slug: "..." }, ...]
```

### With slug mode

```ts
// GitHub-style slugs
const result = compile(md, { slug: { mode: "github" } });

// Unicode-friendly slugs
const result2 = compile(md, { slug: { mode: "unicode" } });
```

### With caching

```ts
const result = compile(md, {
  cache: { enabled: true, dir: ".cache/unifast" },
});
```

### With plugins

```ts
import { compile } from "@unifast/node";
import { gfm } from "@unifast/plugin-gfm";
import { frontmatter } from "@unifast/plugin-frontmatter";
import { toc } from "@unifast/plugin-toc";
import { sanitize } from "@unifast/plugin-sanitize";
import { syntect } from "@unifast/plugin-syntect";

const result = compile(md, {
  plugins: [
    gfm(),
    frontmatter(),
    toc({ maxDepth: 3 }),
    sanitize(),
    syntect(),
  ],
});
```

### HAST transform plugin

```ts
import { compile } from "@unifast/node";
import type { UnifastPlugin, HastRoot } from "@unifast/core";

const addClassPlugin: UnifastPlugin = {
  name: "add-class",
  hastTransform(hast: HastRoot): HastRoot {
    for (const child of hast.children) {
      if (child.type === "element" && child.tagName === "p") {
        const classes = (child.properties.className as string[]) ?? [];
        child.properties.className = [...classes, "prose"];
      }
    }
    return hast;
  },
};

const result = compile("Hello world", { plugins: [addClassPlugin] });
// <p class="prose">Hello world</p>
```

### Error handling

```ts
import { compile } from "@unifast/node";
import { ParseError, CompileError } from "@unifast/node";

try {
  const result = compile(input);
  for (const diag of result.diagnostics) {
    if (diag.level === "error") {
      console.error(`Error: ${diag.message}`);
    } else {
      console.warn(`Warning: ${diag.message}`);
    }
  }
} catch (err) {
  if (err instanceof ParseError) {
    console.error(`Parse failed: ${err.message}`);
  }
}
```

### Performance measurement

```ts
const result = compile(largeDocument);
const total = result.stats.parseMs + result.stats.transformMs + result.stats.emitMs;
console.log(`Total: ${total.toFixed(2)}ms`);
console.log(`  Parse:     ${result.stats.parseMs.toFixed(2)}ms`);
console.log(`  Transform: ${result.stats.transformMs.toFixed(2)}ms`);
console.log(`  Emit:      ${result.stats.emitMs.toFixed(2)}ms`);
```

## `CompileOptions` Reference

See [`@unifast/core` Overview](/docs/packages/core/overview) for the full `CompileOptions` type definition.
