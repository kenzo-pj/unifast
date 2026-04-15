---
title: "excerpt()"
description: "Extrahiert einen Auszug aus dem Dokumentinhalt."
---

```ts
import { excerpt } from "@unifast/node";
```

## Signatur

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfiguration für die Auszugs-Extraktion

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Kommentar-Marker, der den Auszug vom Rest trennt |
| `fallbackParagraphs` | `number` | `1` | Anzahl der führenden Absätze, die als Auszug verwendet werden, wenn kein Trennzeichen gefunden wird |
| `fallbackCharacters` | `number` | `undefined` | Maximale Zeichenanzahl für den Fallback-Auszug (kürzt an Wortgrenzen) |

Sind sowohl `fallbackParagraphs` als auch `fallbackCharacters` gesetzt, hat `fallbackParagraphs` Vorrang.

## Rückgabewert

Das Plugin fügt dem Kompilierungsergebnis eine `excerpt`-Eigenschaft hinzu:

| Eigenschaft | Typ | Beschreibung |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Aus dem Dokument extrahierter Klartext-Auszug |

## Verwendung

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

## Beispiele

### Mit Trenn-Marker

Platzieren Sie einen `<!-- more -->`-Kommentar in Ihrem Markdown, um explizit zu markieren, wo der Auszug endet:

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

### Fallback auf den ersten Absatz

Wird kein Trenn-Marker gefunden, greift das Plugin auf die Extraktion der ersten N Absätze zurück:

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

### Fallback auf Zeichenbegrenzung

Kürzt den Auszug auf eine maximale Zeichenlänge und bricht dabei an einer Wortgrenze um:

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
