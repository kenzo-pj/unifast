---
title: "readingTime()"
description: "Estima o tempo de leitura do documento e o inclui no resultado da compilação."
---

```ts
import { readingTime } from "@unifast/node";
```

## Assinatura

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parâmetros

### options?

Configuração para a estimativa de tempo de leitura

| Propriedade | Tipo | Padrão | Descrição |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Palavras por minuto para texto latino |
| `cjkCharsPerMinute` | `number` | `500` | Caracteres por minuto para texto CJK |

## Retorna

O plugin adiciona uma propriedade `readingTime` ao resultado da compilação:

| Propriedade | Tipo | Descrição |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Tempo estimado de leitura em minutos (mínimo 1, arredondado para o 0,5 mais próximo para cima) |
| `result.readingTime.words` | `number` | Contagem total de palavras (palavras latinas + caracteres CJK) |

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

## Exemplos

### Palavras por minuto customizadas

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

### Conteúdo CJK

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

### Texto misto latino e CJK

O tempo de leitura é calculado separadamente para palavras latinas e caracteres CJK e depois combinado. Code blocks são excluídos da contagem de palavras.

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
