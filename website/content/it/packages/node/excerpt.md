---
title: "excerpt()"
description: "Estrae un estratto dal contenuto del documento."
---

```ts
import { excerpt } from "@unifast/node";
```

## Firma

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parametri

### options?

Configurazione per l'estrazione dell'estratto

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Marcatore di commento che separa l'estratto dal resto |
| `fallbackParagraphs` | `number` | `1` | Numero di paragrafi iniziali da utilizzare come estratto quando il separatore non viene trovato |
| `fallbackCharacters` | `number` | `undefined` | Lunghezza massima in caratteri per l'estratto di fallback (tronca al confine di parola) |

Quando sono impostati sia `fallbackParagraphs` che `fallbackCharacters`, `fallbackParagraphs` ha la precedenza.

## Valore restituito

Il plugin aggiunge una proprietà `excerpt` al risultato della compilazione:

| Proprietà | Tipo | Descrizione |
|-----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Estratto in testo semplice ricavato dal documento |

## Utilizzo

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

## Esempi

### Con il marcatore separatore

Posiziona un commento `<!-- more -->` nel tuo Markdown per indicare esplicitamente dove termina l'estratto:

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

### Fallback al primo paragrafo

Quando il marcatore separatore non viene trovato, il plugin ripiega sull'estrazione dei primi N paragrafi:

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

### Fallback al limite di caratteri

Tronca l'estratto a una lunghezza massima in caratteri, interrompendo al confine di parola:

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
