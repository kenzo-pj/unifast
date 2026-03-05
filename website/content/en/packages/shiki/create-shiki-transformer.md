---
title: "createShikiTransformer()"
description: "Create a standalone Shiki transformer for lower-level control. Unlike createShikiPlugin(), this returns a raw transformer object that you can apply manually to HAST trees."
---

```ts
import { createShikiTransformer } from "@unifast/shiki";
```

### Signature

```ts
async function createShikiTransformer(
  options?: ShikiTransformerOptions,
): Promise<ShikiTransformer>
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options?` | `ShikiTransformerOptions` | Shiki configuration (themes, languages) |

#### `ShikiTransformerOptions`

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `themes` | `BundledTheme[]` | `["github-dark"]` | Shiki themes to load |
| `defaultTheme` | `BundledTheme` | First theme in `themes` | Default theme for rendering |
| `langs` | `BundledLanguage[]` | `[]` | Languages to load. Only loaded languages will be highlighted. |

> `BundledTheme` and `BundledLanguage` are types from the `shiki` package.

### Returns

`Promise<ShikiTransformer>` - An object with a `transform(hast: HastRoot): HastRoot` method.

## Usage

### Manual HAST transformation

```ts
import { compile } from "@unifast/node";
import { createShikiTransformer } from "@unifast/shiki";
import type { HastRoot } from "@unifast/shiki";

const transformer = await createShikiTransformer({
  themes: ["github-dark"],
  langs: ["typescript"],
});

// Get HAST output from compiler
const result = compile(md, { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);

// Apply Shiki highlighting
const highlighted = transformer.transform(hast);
```

### Convert to HTML after transform

```ts
import { compile } from "@unifast/node";
import { createShikiTransformer, hastToHtml } from "@unifast/shiki";
import type { HastRoot } from "@unifast/shiki";

const transformer = await createShikiTransformer({
  themes: ["github-dark"],
  langs: ["typescript", "rust"],
});

const result = compile(md, { outputKind: "hast" });
const hast: HastRoot = JSON.parse(result.output as string);
const highlighted = transformer.transform(hast);
const html = hastToHtml(highlighted);
```

### Use in a custom plugin

```ts
import { createShikiTransformer } from "@unifast/shiki";
import type { UnifastPlugin } from "@unifast/core";

async function createMyPlugin(): Promise<UnifastPlugin> {
  const transformer = await createShikiTransformer({
    themes: ["github-dark"],
    langs: ["typescript"],
  });

  return {
    name: "my-shiki-plugin",
    options: { highlight: { enabled: false } },
    hastTransform: (hast) => {
      // Custom pre-processing here...
      return transformer.transform(hast);
    },
  };
}
```
