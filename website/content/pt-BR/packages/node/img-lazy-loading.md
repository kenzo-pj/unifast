---
title: "imgLazyLoading()"
description: 'Adiciona o atributo loading="lazy" às imagens para carregamento postergado.'
---

```ts
import { imgLazyLoading } from "@unifast/node";
```

## Assinatura

```ts
function imgLazyLoading(options?: ImgLazyLoadingPluginOptions): UnifastPlugin
```

## Parâmetros

### options?

Configuração para o comportamento de lazy loading

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `skipFirst` | `number` | `0` | Número de imagens a pular (ex.: pular a imagem hero) |

O plugin adiciona ambos os atributos `loading="lazy"` e `decoding="async"` aos elementos `<img>` correspondentes, incluindo imagens aninhadas dentro de outros elementos.

## Uso

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Photo 1](photo1.jpg)

![Photo 2](photo2.jpg)

![Photo 3](photo3.jpg)
`;

const result = compile(md, {
  plugins: [imgLazyLoading()],
});

// All images get loading="lazy" and decoding="async":
// <img src="photo1.jpg" alt="Photo 1" loading="lazy" decoding="async">
// <img src="photo2.jpg" alt="Photo 2" loading="lazy" decoding="async">
// <img src="photo3.jpg" alt="Photo 3" loading="lazy" decoding="async">
```

## Exemplos

### Pular a primeira imagem (padrão hero)

A primeira imagem em uma página geralmente é uma hero ou banner que deve carregar de forma eager. Use `skipFirst` para excluí-la do lazy loading:

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const md = `
![Hero banner](hero.jpg)

Some introductory content...

![Diagram](diagram.jpg)

More content...

![Screenshot](screenshot.jpg)
`;

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 1,
    }),
  ],
});

// First image loads eagerly (no loading attribute):
// <img src="hero.jpg" alt="Hero banner">
//
// Remaining images are lazy loaded:
// <img src="diagram.jpg" alt="Diagram" loading="lazy" decoding="async">
// <img src="screenshot.jpg" alt="Screenshot" loading="lazy" decoding="async">
```

### Pular múltiplas imagens above-the-fold

```ts
import { compile, imgLazyLoading } from "@unifast/node";

const result = compile(md, {
  plugins: [
    imgLazyLoading({
      skipFirst: 3, // skip the first 3 images
    }),
  ],
});
```
