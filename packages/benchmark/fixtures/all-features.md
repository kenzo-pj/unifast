---
title: Unifast Feature Showcase
author: unifast-team
date: 2026-03-08
tags: [benchmark, markdown, features]
description: A document that exercises all plugin-equivalent features.
---

# Introduction {#intro}

Unifast is a high-performance Markdown compiler :rocket: built in Rust. It supports
GFM, math, directives, and many more features out of the box. This document
showcases every feature for benchmarking purposes.

<!-- This HTML comment should be removed by the comment-removal pass -->

<!-- more -->

## GFM Features {#gfm}

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Tables | Done | Full GFM support |
| Task lists | Done | Checked and unchecked |
| Strikethrough | Done | Double tilde syntax |
| Footnotes | Done | Reference-style |
| Autolink | Done | URL detection |

### Task Lists

- [x] Implement parser
- [x] Add transform passes
- [ ] Write documentation
- [ ] ~~Remove legacy code~~ (already done)

### Footnotes

Unifast supports GFM footnotes[^1] for adding references. Multiple footnotes
can be used throughout the document[^2], and they can contain longer content[^longnote].

[^1]: This is a simple footnote.
[^2]: Footnotes are rendered at the bottom of the document.
[^longnote]: This is a longer footnote with multiple paragraphs.

    It can span multiple lines and include **formatting**.

## Math {#math}

Inline math like $E = mc^2$ and $\sum_{i=1}^{n} x_i$ can appear within text.
The quadratic formula is $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$.

Display math is also supported:

$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

$$
\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t}
$$

## GitHub Alerts {#alerts}

> [!NOTE]
> This is a note alert. Use it for additional information.

> [!TIP]
> This is a tip alert. Use it for helpful suggestions.

> [!IMPORTANT]
> This is an important alert. Use it for crucial details.

> [!WARNING]
> This is a warning alert. Use it for potential issues.

> [!CAUTION]
> This is a caution alert. Use it for dangerous actions.

## Directives {#directives}

:::note
This is a note directive container. It wraps content in a styled block.
:::

:::warning
Be careful when using raw HTML in Markdown -- it can introduce security
vulnerabilities if not properly sanitized.
:::

:::tip{.custom-class}
Use directives to create reusable content blocks in your documentation.
:::

## Wiki Links {#wiki-links}

You can link to other pages using [[Getting Started]] or with custom display
text like [[API Reference|the API docs]]. Internal links such as
[[Configuration]], [[Plugins]], and [[Deployment Guide]] are resolved
automatically.

## Emoji :tada: {#emoji}

Emoji shortcodes like :smile:, :heart:, :fire:, :warning:, and :star: are
converted to Unicode characters. You can use them inline -- for example,
"this feature is :100: percent complete" or "check out our :sparkles: new
release."

## Smartypants {#smartypants}

Smartypants converts ASCII punctuation to typographic equivalents:

- Straight quotes "like these" become curly quotes.
- Single quotes 'like these' also become curly.
- Double dashes -- become en-dashes.
- Triple dashes --- become em-dashes.
- Ellipses... are converted automatically.
- Backtick quotes ``like these'' also work.

He said, "It's a beautiful day," and she replied, "Indeed it is."

## Definition Lists {#definition-lists}

Markdown
: A lightweight markup language for creating formatted text using a plain-text editor.

HTML
: The standard markup language for documents designed to be displayed in a web browser.

CSS
: A style sheet language used for describing the presentation of a document written in HTML.

Rust
: A multi-paradigm, general-purpose programming language that emphasizes performance,
  type safety, and concurrency.

## Ruby Annotations {#ruby}

Ruby annotations are used for East Asian typography:

{漢字|かんじ}は日本語の表記に使われる文字です。

{東京|とうきょう}は日本の首都であり、{大阪|おおさか}は関西の中心都市です。

The {明治|めいじ} era marked a significant period in Japanese history.

## CJK Text {#cjk}

Unifastは高性能なMarkdownコンパイラです。Rustで書かれており、既存のJavaScript
ベースのツールよりも大幅に高速です。

中文支持也很重要。Unifast可以正确处理中文、日文和韩文的文本排版。

한국어 텍스트도 올바르게 처리됩니다. CJK 문자 사이의 간격은 자동으로 조정됩니다.

Mixed content with English and 日本語 and 中文 should be handled seamlessly
with proper spacing between scripts.

## Abbreviations {#abbreviations}

The HTML specification is maintained by the W3C. It defines how CSS and
JavaScript interact with the DOM. Modern web development relies heavily on
these technologies, along with APIs like the WebGL and WebRTC standards.

*[HTML]: HyperText Markup Language
*[W3C]: World Wide Web Consortium
*[CSS]: Cascading Style Sheets
*[DOM]: Document Object Model
*[WebGL]: Web Graphics Library
*[WebRTC]: Web Real-Time Communication
*[API]: Application Programming Interface
*[APIs]: Application Programming Interfaces

## Custom Heading IDs {#custom-ids}

### Performance Benchmarks {#perf-bench}

### Configuration Guide {#config-guide}

### Migration from v1 to v2 {#migration-v1-v2}

## Code Blocks with Meta {#code-meta}

```javascript title="hello.js"
console.log("Hello, World!");
```

```rust title="main.rs" {2-4}
fn main() {
    let message = "Hello from Rust!";
    println!("{}", message);
}
```

```python title="example.py" showLineNumbers
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

for i in range(10):
    print(fibonacci(i))
```

## Images and Figures {#images}

![Architecture diagram of the Unifast compiler pipeline](https://example.com/unifast-arch.png)

![Performance comparison chart](https://example.com/benchmark-chart.png)

![The Rust logo](https://example.com/rust-logo.svg)

## Accessible Emoji Content {#accessible-emoji}

The release was a huge success! 🎉 Our team worked hard 💪 to deliver
this on time. Special thanks to all contributors 🙏 who made this possible.

Performance improved by 10x 🚀 compared to the previous version. The new
parser is blazingly fast ⚡ and memory efficient 🧠. We're excited 😊 about
the future of this project.

Warning ⚠️: Breaking changes ahead! Please read the migration guide 📖
before upgrading. If you encounter any issues 🐛, please file a report.

## External Links {#external-links}

Check out these resources:

- [Rust Programming Language](https://www.rust-lang.org/)
- [MDX Documentation](https://mdxjs.com/)
- [unified ecosystem](https://unifiedjs.com/)
- [Node.js](https://nodejs.org/en/docs/)
- [CommonMark Specification](https://spec.commonmark.org/)

## Excerpt and Comment Removal {#excerpt-comments}

<!-- This is another HTML comment that should be stripped -->

This section contains both excerpt markers and HTML comments. The excerpt
feature extracts a summary from the content.

<!-- TODO: Add more examples here -->

## Breaks and Soft Line Breaks {#breaks}

This line has a soft break
here in the middle of the paragraph.
Each newline becomes a `<br>` when breaks mode is enabled.
This is useful for poetry or addresses.

## Line Numbers and Highlighting {#line-numbers}

```typescript
import { compile } from "@unifast/node";

const result = compile("# Hello World", {
  gfm: { tables: true },
  frontmatter: { yaml: true },
  sanitize: { enabled: true },
});

console.log(result.html);
```

```css
.container {
  display: flex;
  gap: 1rem;
  padding: 2rem;
}

.card {
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
```

## Sections and Headings {#sections}

### Getting Started

This section covers the basics of using Unifast in your project.

### Advanced Configuration

This section covers advanced topics like custom passes and plugins.

### Troubleshooting

Common issues and their solutions are documented here.

## Performance Table {#performance}

| Compiler | Simple (ops/s) | Readme (ops/s) | Large (ops/s) |
|----------|---------------|----------------|---------------|
| Unifast | 125,000 | 42,000 | 8,500 |
| unified | 18,000 | 6,200 | 1,100 |
| markdown-it | 22,000 | 7,800 | 1,400 |
| marked | 35,000 | 12,000 | 2,300 |

> **Note:** Benchmarks measured on an M2 MacBook Pro with Node.js v22.
> Results may vary depending on document complexity and enabled features.
