---
title: "Using MDX"
description: "Write JSX expressions and import components in Markdown files"
---

## Using MDX

MDX lets you use JSX expressions and import statements inside Markdown. unifast compiles MDX to JavaScript modules that can be rendered with React or other JSX runtimes.

### Installation

```sh
npm install @unifast/node @unifast/mdx
```

### Basic Usage

```ts
import { compileMdx } from "@unifast/mdx";

const source = `
# Hello

<Counter initial={0} />

export const meta = { title: "My Page" };
`;

const result = compileMdx(source);
// result.output is a JavaScript module string
```

### How MDX Works

MDX extends Markdown with two capabilities:

1. **JSX expressions** - Use components inline with your content.
2. **ESM imports/exports** - Import components and export metadata.

The compiler processes MDX in these steps:

```
MDX source
  → Parse (Markdown + JSX + ESM)
  → MdAst with JSX/ESM nodes
  → Lower to HAst
  → Emit as JavaScript module
```

The output is a JavaScript module with a default export function that accepts a `components` prop for component injection.

### Using with React

```ts
import { compileMdx } from "@unifast/mdx";
import { compileToReact } from "@unifast/plugin-react";

const source = `# Hello\n\nThis is **MDX**.`;

const result = compileMdx(source);
const Component = compileToReact(result);

// Render in your React app
<Component components={{ h1: MyHeading }} />
```

### Frontmatter in MDX

Combine MDX with the frontmatter plugin to extract metadata:

```ts
import { compileMdx } from "@unifast/mdx";
import { frontmatter } from "@unifast/plugin-frontmatter";

const source = `---
title: My Article
author: Jane
---

# {frontmatter.title}

Written by {frontmatter.author}.
`;

const result = compileMdx(source, {
  plugins: [frontmatter()],
});

console.log(result.frontmatter);
// { title: "My Article", author: "Jane" }
```

### See Also

- [compileMdx()](/docs/packages/mdx/compile-mdx) - Full API reference
- [React Integration](/docs/guides/react) - Rendering MDX with React
