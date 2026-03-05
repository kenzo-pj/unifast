---
title: "codeImport()"
description: "Import code from external files into fenced code blocks."
---

```ts
import { codeImport } from "@unifast/node";
```

### Signature

```ts
function codeImport(options?: CodeImportPluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `CodeImportPluginOptions` | Configuration for code import behavior |

#### `CodeImportPluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `rootDir` | `string` | — | Root directory for resolving file paths. When omitted, paths are resolved relative to the Markdown file. |

### Returns

`UnifastPlugin`

## Usage

### Basic usage

Reference an external file using the `file` attribute in a fenced code block:

````md
```ts file=src/example.ts
```
````

```ts
import { compile, codeImport } from "@unifast/node";

const md = `
# API Example

\`\`\`ts file=src/example.ts
\`\`\`
`;

const result = compile(md, { plugins: [codeImport()] });
// The code block is replaced with the contents of src/example.ts
```

### Custom rootDir

```ts
import { compile, codeImport } from "@unifast/node";

const result = compile(md, {
  plugins: [codeImport({ rootDir: "/path/to/project" })],
});
// file=src/example.ts resolves to /path/to/project/src/example.ts
```

### Multiple imports

````md
# Code Examples

## TypeScript

```ts file=src/index.ts
```

## Configuration

```json file=tsconfig.json
```

## Styles

```css file=src/styles/main.css
```
````

```ts
import { compile, codeImport } from "@unifast/node";

const result = compile(md, {
  plugins: [codeImport({ rootDir: "./examples" })],
});
```

### Combined with other plugins

```ts
import { compile, codeImport, gfm, frontmatter } from "@unifast/node";

const result = compile(md, {
  plugins: [codeImport({ rootDir: "./src" }), gfm(), frontmatter()],
});
```
