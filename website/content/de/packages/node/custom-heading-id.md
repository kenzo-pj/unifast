---
title: "customHeadingId()"
description: "Setzt benutzerdefinierte IDs für Überschriften mithilfe der Syntax {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Signatur

```ts
function customHeadingId(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Beispiele

### Benutzerdefinierte ID

Verwenden Sie die Syntax `{#custom-id}` am Ende einer Überschrift, um ein bestimmtes `id`-Attribut zuzuweisen. Der Klammer-Block wird aus dem gerenderten Text entfernt:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Überschrift ohne benutzerdefinierte ID

Überschriften ohne die Syntax `{#...}` bleiben unverändert. Sie verwenden den Standard-Slug, der aus dem Überschriftentext generiert wird:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Klassen und beliebige Attribute

Die Klammer-Syntax unterstützt auch `.class`- und `key=value`-Notation:

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
