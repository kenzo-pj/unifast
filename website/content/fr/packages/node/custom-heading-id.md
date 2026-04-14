---
title: "customHeadingId()"
description: "Définit des identifiants personnalisés sur les titres via la syntaxe {#custom-id}."
---

```ts
import { customHeadingId } from "@unifast/node";
```

## Signature

```ts
function customHeadingId(): UnifastPlugin
```

## Paramètres

Aucun.

## Utilisation

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `# Introduction {#intro}`;

const result = compile(md, {
  plugins: [customHeadingId()],
});
// The heading gets id="intro" instead of the auto-generated slug
```

## Exemples

### Identifiant personnalisé

Utilisez la syntaxe `{#custom-id}` à la fin d'un titre pour lui attribuer un attribut `id` précis. Le bloc entre accolades est retiré du texte rendu :

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started {#getting-started}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Titre sans identifiant personnalisé

Les titres dépourvus de la syntaxe `{#...}` restent inchangés. Ils utilisent le slug par défaut, généré à partir du texte du titre :

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## Getting Started`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="getting-started">Getting Started</h2>
```

### Classes et attributs arbitraires

La syntaxe entre accolades prend également en charge la notation `.classe` et `clé=valeur` :

```ts
import { compile, customHeadingId } from "@unifast/node";

const md = `## API Reference {#api .docs data-level=2}`;

const result = compile(md, { plugins: [customHeadingId()] });
console.log(result.output);
// <h2 id="api" class="docs" data-level="2">API Reference</h2>
```
