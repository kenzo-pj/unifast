---
title: "minify()"
description: "Minifie la sortie HTML en supprimant les espaces superflus."
---

```ts
import { minify } from "@unifast/node";
```

## Signature

```ts
function minify(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

```ts
import { compile, minify } from "@unifast/node";

const md = `# Hello

Some   text   with   extra   spaces.`;

const result = compile(md, {
  plugins: [minify()],
});
// Whitespace is collapsed and unnecessary nodes are removed
```

## Exemples

### Minification de base

Le plugin `minify()` réduit les suites d'espaces consécutifs en un espace unique, supprime les commentaires HTML, retire les nœuds texte ne contenant que des espaces entre les éléments de bloc, et supprime les attributs `class` et `style` vides :

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

### Le contenu préformaté est préservé

Les espaces à l'intérieur des blocs `<pre>` et `<code>` restent intacts, afin que la mise en forme du code ne soit jamais altérée :

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
