---
title: "minify()"
description: "अनावश्यक whitespace हटाकर HTML output को minify करें।"
---

```ts
import { minify } from "@unifast/node";
```

## Signature

```ts
function minify(): UnifastPlugin
```

## Parameters

कोई नहीं।

## उपयोग

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace collapse हो जाता है और अनावश्यक nodes हटा दिए जाते हैं
```

## उदाहरण

### मूल minification

`minify()` plugin लगातार whitespace characters को एकल spaces में collapse करता है, HTML comments हटाता है, block elements के बीच whitespace-only text nodes को strip करता है, और खाली `class` और `style` attributes को हटाता है:

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello World

This   has   extra   whitespace.

<!-- This comment is removed -->

Another paragraph.`;

const result = compile(md, { plugins: [minify()] });
console.log(result.output);
// <h1>Hello World</h1><p>This has extra whitespace.</p><p>Another paragraph.</p>
```

### Preformatted content बरकरार रहती है

`<pre>` और `<code>` blocks के अंदर whitespace अछूता रहता है, इसलिए code formatting कभी नहीं टूटती:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// <pre><code> block के अंदर whitespace बिल्कुल लिखा हुआ preserve होता है
```
