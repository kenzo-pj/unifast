---
title: "Quick Start"
description: "Get up and running with unifast in minutes"
---

## Quick Start

Install unifast and compile your first Markdown document in under a minute.

### Installation

```sh
npm install @unifast/node
```

### Basic Usage

```ts
import { compile } from "@unifast/node";

const result = compile("# Hello, unifast!\n\nThis is **Markdown**.");

console.log(result.html);
// <h1>Hello, unifast!</h1>
// <p>This is <strong>Markdown</strong>.</p>
```

That's it. One import, one function call, HTML output.

### Adding Plugins

Plugins extend the compiler with additional features. Each plugin is a separate package that configures built-in passes.

```ts
import { compile } from "@unifast/node";
import { frontmatter } from "@unifast/plugin-frontmatter";
import { gfm } from "@unifast/plugin-gfm";

const source = `---
title: My Post
date: 2025-01-15
---

# My Post

A table:

| Feature | Status |
|---------|--------|
| GFM     | Yes    |

- [x] Task complete
- [ ] Task pending
`;

const result = compile(source, {
  plugins: [frontmatter(), gfm()],
});

console.log(result.frontmatter);
// { title: "My Post", date: "2025-01-15" }

console.log(result.html);
// Rendered HTML with GFM table and task list
```

### Adding Syntax Highlighting

```ts
import { compile } from "@unifast/node";
import { syntect } from "@unifast/plugin-syntect";

const result = compile(
  '```js\nconsole.log("highlighted");\n```',
  { plugins: [syntect()] }
);
// Code block with syntax highlighting classes
```

### What's Next

- [Key Concepts](/docs/introduction/key-concepts) - Understand the compilation pipeline and architecture.
- [Syntax Highlighting](/docs/guides/syntax-highlighting) - Configure code block highlighting in detail.
- [compile()](/docs/packages/node/compile) - Full API reference for the compile function.
