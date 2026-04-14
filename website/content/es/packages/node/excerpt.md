---
title: "excerpt()"
description: "Extrae un extracto del contenido del documento."
---

```ts
import { excerpt } from "@unifast/node";
```

## Firma

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parámetros

### options?

Configuración de la extracción de extractos

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Marcador de comentario que separa el extracto del resto |
| `fallbackParagraphs` | `number` | `1` | Número de párrafos iniciales a usar como extracto cuando no se encuentra separador |
| `fallbackCharacters` | `number` | `undefined` | Longitud máxima en caracteres para el extracto de reserva (trunca en el límite de palabra) |

Cuando `fallbackParagraphs` y `fallbackCharacters` están configurados a la vez, `fallbackParagraphs` tiene prioridad.

## Retorna

El plugin añade una propiedad `excerpt` al resultado de compilación:

| Propiedad | Tipo | Descripción |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Extracto de texto plano extraído del documento |

## Uso

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

## Ejemplos

### Con marcador separador

Coloca un comentario `<!-- more -->` en tu Markdown para marcar explícitamente dónde termina el extracto:

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

### Fallback al primer párrafo

Cuando no se encuentra un marcador separador, el plugin recurre a extraer los primeros N párrafos:

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

### Fallback a un límite de caracteres

Trunca el extracto a una longitud máxima de caracteres, rompiendo en el límite de palabra:

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
