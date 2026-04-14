---
title: "minify()"
description: "Minimiert die HTML-Ausgabe durch Entfernen unnötiger Leerzeichen."
---

```ts
import { minify } from "@unifast/node";
```

## Signatur

```ts
function minify(): UnifastPlugin
```

## Parameter

Keine.

## Verwendung

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Beispiele

### Einfache Minimierung

Das `minify()`-Plugin fasst aufeinanderfolgende Leerzeichen zu einzelnen Leerzeichen zusammen, entfernt HTML-Kommentare, entfernt reine Leerzeichen-Textknoten zwischen Blockelementen und entfernt leere `class`- und `style`-Attribute:

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

### Vorformatierte Inhalte bleiben erhalten

Leerzeichen innerhalb von `<pre>`- und `<code>`-Blöcken bleiben unberührt, sodass die Code-Formatierung niemals zerstört wird:

```ts
import { compile, minify } from "@unifast/node";

const md = `\`\`\`
  function hello() {
    return "world";
  }
\`\`\``;

const result = compile(md, { plugins: [minify()] });
// Whitespace inside the <pre><code> block is preserved exactly as written
```
