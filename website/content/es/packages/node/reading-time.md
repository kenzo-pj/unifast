---
title: "readingTime()"
description: "Estima el tiempo de lectura del documento y lo incluye en el resultado de compilación."
---

```ts
import { readingTime } from "@unifast/node";
```

## Firma

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parámetros

### options?

Configuración de la estimación del tiempo de lectura

| Propiedad | Tipo | Por defecto | Descripción |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Palabras por minuto para texto latino |
| `cjkCharsPerMinute` | `number` | `500` | Caracteres por minuto para texto CJK |

## Retorna

El plugin añade una propiedad `readingTime` al resultado de compilación:

| Propiedad | Tipo | Descripción |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Tiempo estimado de lectura en minutos (mínimo 1, redondeado hacia arriba al 0,5 más cercano) |
| `result.readingTime.words` | `number` | Total de palabras (palabras latinas + caracteres CJK) |

## Uso

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# My Article

This is a short article with some content that demonstrates
reading time estimation.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

console.log(result.readingTime);
// { minutes: 1, words: 16 }
```

## Ejemplos

### Palabras por minuto personalizadas

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // slower reading speed
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### Contenido CJK

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // adjust for CJK reading speed
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Texto mixto latino y CJK

El tiempo de lectura se calcula por separado para las palabras latinas y los caracteres CJK, y luego se combinan. Los bloques de código se excluyen del conteo de palabras.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Latin words counted at 200 WPM, CJK characters at 500 CPM
console.log(result.readingTime);
```
