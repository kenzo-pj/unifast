---
title: "excerpt()"
description: "Extrai um trecho do conteúdo do documento."
---

```ts
import { excerpt } from "@unifast/node";
```

## Assinatura

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Parâmetros

### options?

Configuração para a extração de trecho

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Marcador de comentário que separa o trecho do restante |
| `fallbackParagraphs` | `number` | `1` | Número de parágrafos iniciais a usar como trecho quando nenhum separador é encontrado |
| `fallbackCharacters` | `number` | `undefined` | Comprimento máximo de caracteres para o trecho de fallback (trunca em uma fronteira de palavra) |

Quando ambos `fallbackParagraphs` e `fallbackCharacters` estão definidos, `fallbackParagraphs` tem precedência.

## Retorna

O plugin adiciona uma propriedade `excerpt` ao resultado da compilação:

| Propriedade | Tipo | Descrição |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Trecho de texto puro extraído do documento |

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

## Exemplos

### Com marcador separador

Coloque um comentário `<!-- more -->` no seu Markdown para marcar explicitamente onde o trecho termina:

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

### Fallback para o primeiro parágrafo

Quando nenhum marcador separador é encontrado, o plugin faz fallback para extrair os primeiros N parágrafos:

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

### Fallback para limite de caracteres

Trunca o trecho para um comprimento máximo de caracteres, quebrando em uma fronteira de palavra:

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
