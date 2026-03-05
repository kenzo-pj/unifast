---
title: "Key Concepts"
description: "Understand unifast's multi-stage compilation pipeline, plugin system, and how MdAst and HAst transformations work."
---

unifast compiles Markdown through a multi-stage pipeline. Understanding these stages helps you configure the compiler and choose the right plugins.

### Compilation Pipeline

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Each stage transforms the document through an intermediate representation (IR).

### Intermediate Representations

| IR | Name | Purpose |
|----|------|---------|
| **IR0** | MdAst | Markdown structure - headings, paragraphs, lists, code blocks |
| **IR1** | HAst | HTML structure - elements, attributes, text nodes |
| **IR2** | JsAst | MDX only - ESM imports and JSX expressions |

The parser produces **MdAst**, which is then lowered to **HAst** for HTML emission. MDX inputs additionally produce **JsAst** nodes for JavaScript output.

### Passes

Passes are transformations applied to the AST at specific phases. unifast has built-in passes for common tasks:

**MdAst passes** (before lowering to HTML):

- **Normalize** - Consistent structure for downstream passes
- **Slug** - Generate heading IDs from text content
- **TOC** - Extract table of contents from headings
- **Definition Resolution** - Resolve link/image reference definitions

**HAst passes** (after lowering to HTML):

- **Sanitize** - Remove disallowed HTML elements and attributes
- **Highlight** - Apply syntax highlighting to code blocks
- **Cleanup** - Remove unnecessary nodes and whitespace

Passes are ordered by phase - you don't need to worry about execution order.

### Plugins

Plugins are TypeScript packages that configure built-in passes. They don't run arbitrary JavaScript during compilation - instead, they set options that control how the Rust core processes your document.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Each plugin returns a configuration object that is merged into the compile options before the Rust core runs. This keeps the hot path entirely in native code.

### Output Formats

The compiler supports multiple output formats via the `outputKind` option:

| Format | Description |
|--------|-------------|
| `"html"` | HTML string (default) |
| `"hast"` | HAst JSON - the HTML AST for custom rendering |
| `"mdast"` | MdAst JSON - the Markdown AST for analysis |
| `"mdx-js"` | JavaScript module string (MDX only) |

### Diagnostics

The compiler reports issues via the `diagnostics` array in the result. Each diagnostic includes a severity level, message, and optional source span for precise error location.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
