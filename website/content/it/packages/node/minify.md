---
title: "minify()"
description: "Minifica l'output HTML rimuovendo gli spazi bianchi superflui."
---

```ts
import { minify } from "@unifast/node";
```

## Firma

```ts
function minify(): UnifastPlugin
```

## Parametri

Nessuno.

## Utilizzo

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Esempi

### Minificazione di base

Il plugin `minify()` comprime i caratteri di spazio bianco consecutivi in un singolo spazio, rimuove i commenti HTML, elimina i nodi di testo composti solo da spazi bianchi tra gli elementi di blocco e rimuove gli attributi `class` e `style` vuoti:

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

### I contenuti preformattati vengono preservati

Gli spazi bianchi all'interno dei blocchi `<pre>` e `<code>` restano intatti, così la formattazione del codice non viene mai compromessa:

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
