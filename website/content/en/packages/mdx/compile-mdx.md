---
title: "compileMdx()"
description: "Compile MDX input with pre-configured MDX defaults. A convenience wrapper that automatically sets inputKind to mdx."
---

```ts
import { compileMdx } from "@unifast/mdx";
```

### Signature

```ts
function compileMdx(
  input: string,
  options?: Partial<MdxCompileOptions>,
): CompileResult
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `string` | MDX source string |
| `options?` | `Partial<MdxCompileOptions>` | MDX-specific compile options |

#### `MdxCompileOptions`

Extends [`CompileOptions`](/docs/packages/core/overview) with MDX-specific constraints.

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `inputKind` | `"mdx"` | `"mdx"` | Always set to `"mdx"` |
| `outputKind` | `"html" \| "mdxJs"` | `"html"` | Output format (HTML or MDX JS module) |
| *(all other fields)* | - | - | Inherited from [`CompileOptions`](/docs/packages/core/overview) |

### Returns

`CompileResult` - See [`@unifast/core` Overview](/docs/packages/core/overview) for the full type.

## Usage

### Basic MDX compilation

```ts
import { compileMdx } from "@unifast/mdx";

const mdx = `
import { Button } from "./components";

# Welcome

<Button onClick={() => alert("hi")}>Click me</Button>
`;

const result = compileMdx(mdx);
console.log(result.output);
```

### MDX JS output

```ts
import { compileMdx } from "@unifast/mdx";

const result = compileMdx(mdxSource, {
  outputKind: "mdxJs",
});
```

### With plugins

```ts
import { compileMdx } from "@unifast/mdx";
import { frontmatter } from "@unifast/plugin-frontmatter";
import { gfm } from "@unifast/plugin-gfm";

const result = compileMdx(mdxSource, {
  plugins: [frontmatter(), gfm()],
});

console.log(result.frontmatter);
```

### With all options

```ts
import { compileMdx } from "@unifast/mdx";
import { gfm } from "@unifast/plugin-gfm";
import { frontmatter } from "@unifast/plugin-frontmatter";
import { toc } from "@unifast/plugin-toc";

const result = compileMdx(mdxSource, {
  outputKind: "html",
  plugins: [gfm(), frontmatter(), toc()],
  slug: { mode: "github" },
});
```
