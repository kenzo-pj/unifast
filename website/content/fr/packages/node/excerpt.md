---
title: "excerpt()"
description: "Extrait un extrait depuis le contenu du document."
---

```ts
import { excerpt } from "@unifast/node";
```

## Signature

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Paramètres

### options?

Configuration de l'extraction d'extrait

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Marqueur de commentaire qui sépare l'extrait du reste |
| `fallbackParagraphs` | `number` | `1` | Nombre de paragraphes de tête à utiliser comme extrait lorsqu'aucun séparateur n'est trouvé |
| `fallbackCharacters` | `number` | `undefined` | Longueur maximale en caractères pour l'extrait de repli (tronqué sur une limite de mot) |

Lorsque `fallbackParagraphs` et `fallbackCharacters` sont tous deux définis, `fallbackParagraphs` prévaut.

## Valeur de retour

Le plugin ajoute une propriété `excerpt` au résultat de compilation :

| Propriété | Type | Description |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Extrait en texte brut issu du document |

## Utilisation

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Exemples

### Avec un marqueur de séparation

Placez un commentaire `<!-- more -->` dans votre Markdown pour indiquer explicitement la fin de l'extrait :

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### Repli sur le premier paragraphe

Lorsqu'aucun marqueur de séparation n'est trouvé, le plugin se rabat sur l'extraction des N premiers paragraphes :

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Repli sur une limite de caractères

Tronquez l'extrait à une longueur maximale de caractères, en coupant sur une limite de mot :

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
