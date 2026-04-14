---
title: "commentRemoval()"
description: "Entfernt HTML-Kommentare aus der Ausgabe."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Signatur

```ts
function commentRemoval(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `# Hello

<!-- This comment will be removed -->

Some content here.`;

const result = compile(md, {
  plugins: [commentRemoval()],
});
// The HTML comment is stripped from the output
```

## Beispiele

### Einfaches Entfernen von Kommentaren

Alle HTML-Kommentarknoten (`<!-- ... -->`) werden aus dem Ausgabebaum entfernt, einschließlich Kommentaren, die in Blockelementen wie Blockquotes verschachtelt sind:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `First paragraph.

<!-- TODO: add more content -->

Second paragraph.`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <p>First paragraph.</p>
// <p>Second paragraph.</p>
```

### Nicht-Kommentar-HTML bleibt erhalten

Nur Kommentarknoten werden entfernt. Anderes Inline-HTML bleibt unberührt:

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
