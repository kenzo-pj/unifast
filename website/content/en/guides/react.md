---
title: "React Integration"
description: "Render compiled Markdown and MDX content in React applications"
---

## React Integration

`@unifast/plugin-react` provides utilities for rendering unifast output in React applications - from simple HTML rendering to full MDX component mapping.

### Installation

```sh
npm install @unifast/node @unifast/plugin-react
```

### Using hastToReact

Convert the HAst (HTML AST) to React elements. This lets you map HTML elements to custom React components without raw HTML injection.

```tsx
import { compile } from "@unifast/node";
import { hastToReact } from "@unifast/plugin-react";

const result = compile(source, { outputKind: "hast" });
const elements = hastToReact(result.output, {
  components: {
    h1: (props) => <h1 className="heading" {...props} />,
    a: (props) => <a className="link" target="_blank" {...props} />,
    pre: CodeBlock,
  },
});

function Page() {
  return <article>{elements}</article>;
}
```

This approach is safe by default - the AST is converted to React elements without raw HTML.

### Rendering MDX

For MDX content, use `compileToReact` to get a React component directly:

```tsx
import { compileMdx } from "@unifast/mdx";
import { compileToReact } from "@unifast/plugin-react";

const result = compileMdx(source);
const Content = compileToReact(result);

function Page() {
  return (
    <Content
      components={{
        Alert: MyAlertComponent,
        CodeBlock: MyCodeBlock,
      }}
    />
  );
}
```

### Component Mapping

Both `hastToReact` and `compileToReact` accept a `components` map. Use this to replace default HTML elements with custom React components:

```tsx
const components = {
  // Replace headings
  h1: ({ children }) => <h1 className="text-3xl font-bold">{children}</h1>,

  // Custom code blocks
  pre: ({ children, ...props }) => <CodeBlock {...props}>{children}</CodeBlock>,

  // External links open in new tab
  a: ({ href, children }) => (
    <a href={href} target={href?.startsWith("http") ? "_blank" : undefined}>
      {children}
    </a>
  ),
};
```

### Server-Side Rendering

unifast compilation is synchronous and runs in Node.js, making it ideal for SSR:

```tsx
// server.tsx
import { compile } from "@unifast/node";
import { frontmatter } from "@unifast/plugin-frontmatter";
import { hastToReact } from "@unifast/plugin-react";

export async function getStaticProps() {
  const source = await readFile("content/post.md", "utf8");
  const result = compile(source, {
    plugins: [frontmatter()],
    outputKind: "hast",
  });

  return {
    props: {
      hast: result.output,
      meta: result.frontmatter,
    },
  };
}

function Post({ hast, meta }) {
  const content = hastToReact(hast);
  return (
    <article>
      <h1>{meta.title}</h1>
      {content}
    </article>
  );
}
```

### See Also

- [compileToReact()](/docs/plugins/plugin-react/compile-to-react) - API reference
- [hastToReact()](/docs/plugins/plugin-react/hast-to-react) - API reference
- [Using MDX](/docs/guides/mdx) - MDX compilation guide
