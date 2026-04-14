---
title: "readingTime()"
description: "Schätzt die Lesezeit des Dokuments und nimmt sie in das Kompilierungsergebnis auf."
---

```ts
import { readingTime } from "@unifast/node";
```

## Signatur

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Parameter

### options?

Konfiguration für die Schätzung der Lesezeit

| Eigenschaft | Typ | Standard | Beschreibung |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Wörter pro Minute für Text in lateinischer Schrift |
| `cjkCharsPerMinute` | `number` | `500` | Zeichen pro Minute für CJK-Text |

## Rückgabewert

Das Plugin fügt dem Kompilierungsergebnis eine `readingTime`-Eigenschaft hinzu:

| Eigenschaft | Typ | Beschreibung |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Geschätzte Lesezeit in Minuten (Minimum 1, aufgerundet auf die nächsten 0,5) |
| `result.readingTime.words` | `number` | Gesamtwortzahl (lateinische Wörter + CJK-Zeichen) |

## Verwendung

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

## Beispiele

### Benutzerdefinierte Wörter pro Minute

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

### CJK-Inhalte

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

### Gemischter lateinischer und CJK-Text

Die Lesezeit wird für lateinische Wörter und CJK-Zeichen getrennt berechnet und anschließend kombiniert. Code-Blöcke werden von der Wortzählung ausgeschlossen.

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
