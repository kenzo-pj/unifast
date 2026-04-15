---
title: "excerpt()"
description: "document content से एक excerpt निकालें।"
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

Excerpt extraction के लिए Configuration

| Property | Type | Default | विवरण |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Comment marker जो excerpt को बाकी से अलग करता है |
| `fallbackParagraphs` | `number` | `1` | जब कोई separator नहीं मिलता तो excerpt के रूप में उपयोग करने के लिए leading paragraphs की संख्या |
| `fallbackCharacters` | `number` | `undefined` | fallback excerpt के लिए अधिकतम character length (word boundary पर truncate करता है) |

जब `fallbackParagraphs` और `fallbackCharacters` दोनों set होते हैं, तो `fallbackParagraphs` को प्राथमिकता मिलती है।

## Returns

Plugin compile result में एक `excerpt` property जोड़ता है:

| Property | Type | विवरण |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | document से निकाला गया Plain text excerpt |

## उपयोग

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

## उदाहरण

### separator marker के साथ

excerpt कहाँ समाप्त होता है, इसे स्पष्ट रूप से चिह्नित करने के लिए अपने Markdown में एक `<!-- more -->` comment रखें:

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

### पहले paragraph पर fallback

जब कोई separator marker नहीं मिलता, तो plugin पहले N paragraphs को निकालने पर fall back हो जाता है:

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

### character limit पर fallback

एक word boundary पर break करते हुए, excerpt को अधिकतम character length तक truncate करें:

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
