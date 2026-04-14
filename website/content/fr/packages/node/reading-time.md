---
title: "readingTime()"
description: "Estime le temps de lecture du document et l'inclut dans le résultat de compilation."
---

```ts
import { readingTime } from "@unifast/node";
```

## Signature

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Paramètres

### options?

Configuration de l'estimation du temps de lecture

| Propriété | Type | Défaut | Description |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Mots par minute pour le texte latin |
| `cjkCharsPerMinute` | `number` | `500` | Caractères par minute pour le texte CJK |

## Valeur de retour

Le plugin ajoute une propriété `readingTime` au résultat de compilation :

| Propriété | Type | Description |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Temps de lecture estimé en minutes (minimum 1, arrondi au 0,5 supérieur) |
| `result.readingTime.words` | `number` | Nombre total de mots (mots latins + caractères CJK) |

## Utilisation

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

## Exemples

### Nombre de mots par minute personnalisé

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

### Contenu CJK

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

### Texte mixte latin et CJK

Le temps de lecture est calculé séparément pour les mots latins et les caractères CJK, puis combiné. Les blocs de code sont exclus du décompte de mots.

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
