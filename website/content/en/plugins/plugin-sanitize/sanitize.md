---
title: "sanitize()"
description: "Create a sanitization plugin that strips dangerous HTML tags, attributes, and URL protocols from compiled output."
---

```ts
import { sanitize } from "@unifast/plugin-sanitize";
```

### Signature

```ts
function sanitize(options?: SanitizePluginOptions): UnifastPlugin
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `SanitizePluginOptions` | Sanitization configuration |

#### `SanitizePluginOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable or disable sanitization |
| `schema` | `SanitizeSchema` | - | Custom sanitization schema |

#### `SanitizeSchema`

| Property | Type | Description |
|----------|------|-------------|
| `allowedTags` | `string[]` | HTML tag names to allow (all others are stripped) |
| `allowedAttributes` | `Record<string, string[]>` | Map of tag name to allowed attribute names |
| `allowedProtocols` | `Record<string, string[]>` | Map of attribute name to allowed URL protocols |

### Returns

`UnifastPlugin`

## Usage

### Basic usage (default schema)

```ts
import { compile } from "@unifast/node";
import { sanitize } from "@unifast/plugin-sanitize";

const untrustedMd = `
# Hello

<script>alert("xss")</script>

<img src="x" onerror="alert('xss')">

[Click me](javascript:alert('xss'))
`;

const result = compile(untrustedMd, {
  plugins: [sanitize()],
});
// <script> tags and dangerous attributes are removed
```

### Custom allowed tags

```ts
import { compile } from "@unifast/node";
import { sanitize } from "@unifast/plugin-sanitize";

const result = compile(md, {
  plugins: [
    sanitize({
      schema: {
        allowedTags: [
          "h1", "h2", "h3", "h4", "h5", "h6",
          "p", "a", "strong", "em", "code", "pre",
          "ul", "ol", "li", "blockquote", "img",
          "table", "thead", "tbody", "tr", "th", "td",
        ],
      },
    }),
  ],
});
```

### Custom allowed attributes

```ts
import { compile } from "@unifast/node";
import { sanitize } from "@unifast/plugin-sanitize";

const result = compile(md, {
  plugins: [
    sanitize({
      schema: {
        allowedTags: ["a", "img", "p", "h1", "h2", "h3", "code", "pre"],
        allowedAttributes: {
          a: ["href", "title", "target"],
          img: ["src", "alt", "width", "height"],
          code: ["class"],
          pre: ["class"],
        },
      },
    }),
  ],
});
```

### Restrict URL protocols

```ts
import { compile } from "@unifast/node";
import { sanitize } from "@unifast/plugin-sanitize";

const result = compile(md, {
  plugins: [
    sanitize({
      schema: {
        allowedProtocols: {
          href: ["https", "http", "mailto"],
          src: ["https", "http"],
        },
      },
    }),
  ],
});
```

### Disable sanitization

```ts
import { sanitize } from "@unifast/plugin-sanitize";

const result = compile(md, {
  plugins: [sanitize({ enabled: false })],
});
```

### Full custom schema

```ts
import { compile } from "@unifast/node";
import { sanitize } from "@unifast/plugin-sanitize";

const result = compile(md, {
  plugins: [
    sanitize({
      schema: {
        allowedTags: ["p", "a", "strong", "em", "code", "pre", "img"],
        allowedAttributes: {
          a: ["href", "title"],
          img: ["src", "alt"],
          code: ["class"],
          pre: ["class"],
        },
        allowedProtocols: {
          href: ["https", "mailto"],
          src: ["https"],
        },
      },
    }),
  ],
});
```
