---
title: "readingTime()"
description: "Stima il tempo di lettura del documento e lo include nel risultato della compilazione."
---

```ts
import { readingTime } from "@unifast/node";
```

## Firma

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parametri

### options?

Configurazione per la stima del tempo di lettura

| Proprietà | Tipo | Predefinito | Descrizione |
|-----------|------|-------------|-------------|
| `wordsPerMinute` | `number` | `200` | Parole al minuto per il testo latino |
| `cjkCharsPerMinute` | `number` | `500` | Caratteri al minuto per il testo CJK |

## Valore restituito

Il plugin aggiunge una proprietà `readingTime` al risultato della compilazione:

| Proprietà | Tipo | Descrizione |
|-----------|------|-------------|
| `result.readingTime.minutes` | `number` | Tempo di lettura stimato in minuti (minimo 1, arrotondato per eccesso al più vicino 0,5) |
| `result.readingTime.words` | `number` | Conteggio totale delle parole (parole latine + caratteri CJK) |

## Utilizzo

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

## Esempi

### Parole al minuto personalizzate

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

### Contenuti CJK

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

### Testo misto latino e CJK

Il tempo di lettura viene calcolato separatamente per le parole latine e per i caratteri CJK, per poi essere combinato. I blocchi di codice sono esclusi dal conteggio delle parole.

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
