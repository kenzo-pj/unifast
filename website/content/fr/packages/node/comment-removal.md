---
title: "commentRemoval()"
description: "Supprime les commentaires HTML de la sortie."
---

```ts
import { commentRemoval } from "@unifast/node";
```

## Signature

```ts
function commentRemoval(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

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

## Exemples

### Suppression basique des commentaires

Tous les nœuds de commentaire HTML (`<!-- ... -->`) sont retirés de l'arbre de sortie, y compris ceux imbriqués dans des éléments de bloc comme les citations :

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

### Le HTML non commenté est préservé

Seuls les nœuds de commentaires sont supprimés. Les autres éléments HTML en ligne sont conservés intacts :

```ts
import { compile, commentRemoval } from "@unifast/node";

const md = `<!-- hidden -->

<div class="custom">Visible content</div>`;

const result = compile(md, { plugins: [commentRemoval()] });
console.log(result.output);
// <div class="custom">Visible content</div>
```
