---
title: "excerpt()"
description: "Extract an excerpt from the document content."
---

```ts
import { excerpt } from "@unifast/node";
```

## Signature

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parameters

### options?

Configuration for excerpt extraction

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Comment marker that separates the excerpt from the rest |
| `fallbackParagraphs` | `number` | `1` | Number of leading paragraphs to use as excerpt when no separator found |
| `fallbackCharacters` | `number` | `undefined` | Max character length for fallback excerpt (truncates on word boundary) |

When both `fallbackParagraphs` and `fallbackCharacters` are set, `fallbackParagraphs` takes precedence.

## Returns

The plugin adds an `excerpt` property to the compile result:

| Property | Type | Description |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Plain text excerpt extracted from the document |

## Usage

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Examples

### With separator marker

Place a `<!-- more -->` comment in your Markdown to explicitly mark where the excerpt ends:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### Fallback to first paragraph

When no separator marker is found, the plugin falls back to extracting the first N paragraphs:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Fallback to character limit

Truncate the excerpt to a maximum character length, breaking on a word boundary:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
